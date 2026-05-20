use mlua::prelude::*;
use mlua::{UserData, UserDataMethods};
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use chrono::Local;
use std::net::UdpSocket;
use std::process::Command;
use sysinfo::System;

// --- Helper Functions ---

pub fn get_rust_info() -> String {
    if Path::new("Cargo.toml").exists() {
        let output = Command::new("rustc")
            .arg("--version")
            .output()
            .map(|o| {
                let s = String::from_utf8_lossy(&o.stdout);
                s.split_whitespace().nth(1).unwrap_or("").to_string()
            })
            .unwrap_or_else(|_| "Rust".to_string());
        format!("🦀 v{}", output)
    } else {
        "".to_string()
    }
}

pub fn get_path_short() -> String {
    let cwd = env::current_dir().unwrap_or_default();
    let home = env::var("HOME").unwrap_or_default();
    let path_str = cwd.to_string_lossy();

    if path_str == home {
        "~".to_string()
    } else {
        cwd.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "/".to_string())
    }
}

pub fn get_sys_status() -> String {
    let mut sys = System::new_all();
    sys.refresh_cpu_all();
    sys.refresh_memory();

    let cpu = format!("{:.0}", sys.global_cpu_usage());
    let used_gb = sys.used_memory() / 1024 / 1024 / 1024;
    format!("{}% {}GB", cpu, used_gb)
}

pub fn get_local_ip() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    socket.local_addr().ok().map(|addr| addr.ip().to_string())
}

pub fn get_git_branch() -> String {
    let output = Command::new("git")
        .args(["symbolic-ref", "--short", "HEAD"])
        .output();

    match output {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).trim().to_string(),
        _ => String::new(),
    }
}

// --- Shell Configuration ---

#[derive(Clone, Debug)]
pub struct ShellConfig {
    pub raw_prompt_template: String,
    pub aliases: HashMap<String, String>,
    pub abbreviations: HashMap<String, String>,
}

impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            raw_prompt_template: "&(cyan)&(DIR) &(green)➜ ".to_string(),
            aliases: HashMap::new(),
            abbreviations: HashMap::new(),
        }
    }
}

impl UserData for ShellConfig {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("set_prompt", |_, this, input: String| {
            this.raw_prompt_template = input;
            Ok(())
        });

        methods.add_method_mut("alias", |_, this, (name, cmd): (String, String)| {
            this.aliases.insert(name, cmd);
            Ok(())
        });

        methods.add_method_mut("abbr", |_, this, (name, cmd): (String, String)| {
            this.abbreviations.insert(name, cmd);
            Ok(())
        });
    }
}

// --- Core Logic ---

pub fn parse_prompt(template: &str) -> String {
    let mut parsed = template.to_string();
    let home = env::var("HOME").or_else(|_| env::var("USERPROFILE")).unwrap_or_default();
    let user = env::var("USER").or_else(|_| env::var("USERNAME")).unwrap_or_else(|_| "user".into());
    let dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let pretty_dir = dir.to_string_lossy().replace(&home, "~");
    let time = Local::now().format("%H:%M:%S").to_string();

    let tags = [
        ("&(DIR)", pretty_dir),
        ("&(HOST)", get_local_ip().unwrap_or_else(|| "127.0.0.1".into())),
        ("&(GIT)", get_git_branch()),
        ("&(RUST)", get_rust_info()),
        ("&(PATH_SHORT)", get_path_short()),
        ("&(SYSTEM)", get_sys_status()),
        ("&(USER)", user),
        ("&(TIME)", time),
        ("&(blue)", "\x1b[34m".into()),
        ("&(green)", "\x1b[32m".into()),
        ("&(red)", "\x1b[31m".into()),
        ("&(yellow)", "\x1b[33m".into()),
        ("&(cyan)", "\x1b[36m".into()),
        ("&(white)", "\x1b[37m".into()),
        ("&(reset)", "\x1b[0m".into()),
    ];

    for (tag, val) in tags {
        parsed = parsed.replace(tag, &val);
    }
    format!("{}\x1b[0m", parsed)
}

pub fn init_shell() -> Result<(Lua, ShellConfig), Box<dyn std::error::Error>> {
    let lua = Lua::new();
    let config = ShellConfig::default();
    
    // 1. Register the shell object
    let userdata = lua.create_userdata(config.clone())?;
    lua.globals().set("shell", userdata)?;

    // 2. Register if_git as a GLOBAL function
    let if_git = lua.create_function(|_, ()| {
        let is_git = Command::new("git")
            .args(["rev-parse", "--is-inside-work-tree"])
            .output()
            .map(|out| out.status.success())
            .unwrap_or(false);
        Ok(is_git)
    })?;
    lua.globals().set("if_git", if_git)?;

    // 3. Load config file
    let home = env::var("HOME").or_else(|_| env::var("USERPROFILE")).unwrap_or_else(|_| ".".into());
    let config_path = PathBuf::from(home).join(".config/NaShell/config.lua");

    if config_path.exists() {
        let script = std::fs::read_to_string(&config_path)?;
        lua.load(&script).exec()?;
    }

    // 4. Retrieve final config after Lua runs
    let shell_user_data: LuaAnyUserData = lua.globals().get("shell")?;
    let final_config = shell_user_data.borrow::<ShellConfig>()?.clone();
    
    Ok((lua, final_config))
}
