use mlua::prelude::*;
use mlua::{UserData, UserDataMethods};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use chrono::Local;

#[derive(Clone, Debug)]
pub struct ShellConfig {
    pub prompt: String,
    pub raw_prompt_template: String,
    pub aliases: HashMap<String, String>,
    pub abbreviations: HashMap<String, String>,
}

impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            prompt: "> ".to_string(),
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

        methods.add_method("help", |_, _, ()| {
            println!("\n\x1b[1;32mNaShell Commands:\x1b[0m");
            println!("  shell:set_prompt(str) - Tags: &(DIR), &(USER), &(TIME), &(color)");
            println!("  shell:alias(name, cmd)");
            println!("  shell:abbr(name, cmd)");
            println!("  shell:help()\n");
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

pub fn parse_prompt(template: &str) -> String {
    let mut parsed = template.to_string();
    let home = env::var("HOME").or_else(|_| env::var("USERPROFILE")).unwrap_or_default();
    let user = env::var("USER").or_else(|_| env::var("USERNAME")).unwrap_or_else(|_| "user".into());
    let dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let pretty_dir = dir.to_string_lossy().replace(&home, "~");
    let time = Local::now().format("%H:%M:%S").to_string();

    let tags = [
        ("&(DIR)", pretty_dir.to_string()),
        ("&(USER)", user),
        ("&(TIME)", time),
        ("&(blue)", "\x1b[34m".to_string()),
        ("&(green)", "\x1b[32m".to_string()),
        ("&(red)", "\x1b[31m".to_string()),
        ("&(yellow)", "\x1b[33m".to_string()),
        ("&(cyan)", "\x1b[36m".to_string()),
        ("&(white)", "\x1b[37m".to_string()),
        ("&(reset)", "\x1b[0m".to_string()),
    ];

    for (tag, val) in tags {
        parsed = parsed.replace(tag, &val);
    }
    format!("{}\x1b[0m", parsed)
}

pub fn init_shell() -> Result<ShellConfig, Box<dyn std::error::Error>> {
    let lua = Lua::new();
    let config = ShellConfig::default();
    
    let userdata = lua.create_userdata(config)?;
    lua.globals().set("shell", userdata.clone())?;

    let home = env::var("HOME").or_else(|_| env::var("USERPROFILE")).unwrap_or_else(|_| ".".into());
    let config_path = PathBuf::from(home).join(".config/NaShell/config.lua");

    if config_path.exists() {
        let script = std::fs::read_to_string(&config_path)?;
        lua.load(&script).exec()?;
    }

    let final_config = {
        let borrowed = userdata.borrow::<ShellConfig>()?;
        borrowed.clone()
    };
    
    Ok(final_config)
}
