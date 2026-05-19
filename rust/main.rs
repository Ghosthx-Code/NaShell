pub mod command;
pub mod lua;

use std::{process::Command, borrow::Cow, fs, env};
use owo_colors::OwoColorize;
use rustyline::error::ReadlineError;
use rustyline::{Config, Editor, Result, ColorMode};
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter, CmdKind};
use rustyline::{Helper, Completer, Hinter, Validator};

#[derive(Helper, Completer, Hinter, Validator)]
struct MyHelper {
    #[rustyline(Completer)]
    pub completer: rustyline::completion::FilenameCompleter,
    #[rustyline(Hinter)]
    pub hinter: rustyline::hint::HistoryHinter,
    #[rustyline(Validator)]
    pub validator: rustyline::validate::MatchingBracketValidator,
    pub highlighter: MatchingBracketHighlighter,
}

impl Highlighter for MyHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        if line.is_empty() { return Cow::Borrowed(line); }
        let mut highlighted = line.to_string();

               let keywords = [
            "sudo", "git", "cargo", "shell", "python", "rustc", "nvim", "apt", "install", "grep", "vim", "code", "mkdir", "nano", "nala", "lsB", "wifi", "rx", "wget", "wc", "yes", "whoami", "||", "&&",
            "cat", "cp", "mv", "rm", "touch", "ln", "find", "locate", "chmod", "chown", "umask", "df", "du", "free", "top", "htop", "btop", "ps", "kill", "pkill", "killall", "nice", "renice", "nohup", "screen", "watch", "head", "tail", "less", "more", "diff", "patch", "sed", "awk", "sort", "uniq", "tee", "xargs", "alias", "unalias", "history", "type", "whereis", "which", "realpath", "basename", "dirname", "tree", "stat", "file", "mount", "umount", "fdisk", "lsblk", "chroot", "dd", "sync", "tar", "gzip", "gunzip", "bzip2", "zip", "unzip", "7z", "zstd", "xz",
            "ping", "curl", "ssh", "scp", "rsync", "sftp", "ftp", "dig", "nslookup", "host", "ip", "ifconfig", "netstat", "ss", "route", "traceroute", "mtr", "nmap", "tcpdump", "ufw", "iptables", "nft", "aria2c", "socat", "nc", "netcat", "telnet", "nmtui", "nmcli", "iw", "iwconfig", "bluetoothctl", "hcitool",
            "emacs", "neovim", "subl", "micro", "joe", "ex", "view", "bat", "glow", "tldr", "man", "info",
           "fish", "dash", "ash", "csh", "ksh", "tclsh", "expect", "fzf", "skim", "delta", "dust", "duf", "procs", "bottom", "glances", "gping", "dog", "httpie", "xh", "curlie", "lazygit", "lazydocker", "tig", "gitui", "gh", "lab", "tea", "task", "timew", "ledger", "hledger", "calc", "bc", "units", "factor", "base64", "openssl", "gpg", "ssh-keygen", "ssh-copy-id", "shred", "wipe", "srm", "truncate", "fallocate", "ionice", "chrt", "taskset", "lsof", "fuser", "smem", "slabtop", "pcstat", "tiptop", "numastat", "iostat", "mpstat", "sar", "pidstat", "nfsiostat", "cifsiostat", "vmstat", "zpool", "zfs", "btrfs", "lvm", "pvcreate", "vgcreate", "lvcreate", "cryptsetup", "dmsetup", "upadet", "cls", "clear", "cd", "exit", "ls"
        ];

        // 1. Highlight standard commands from the list (Green)
        for word in keywords {
            if line.contains(word) {
                highlighted = highlighted.replace(word, &word.bright_green().to_string());
            }
        }

        // 2. String highlighting (White)
        if line.contains('"') || line.contains('\'') {
             return Cow::Owned(highlighted.white().to_string());
        }

        Cow::Owned(highlighted)
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _forced: CmdKind) -> bool { true }
}

fn handle_source(path: &str) -> std::result::Result<(), String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string().red().to_string())?;
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("export ") {
            let kv_part = &line[7..];
            let kv: Vec<&str> = kv_part.splitn(2, '=').collect();
            if kv.len() == 2 {
                unsafe { env::set_var(kv[0].trim(), kv[1].trim().trim_matches('"')); }
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let config = Config::builder()
        .max_history_size(1000)?
        .color_mode(ColorMode::Enabled) 
        .build();

    let mut rl = Editor::<MyHelper, rustyline::history::DefaultHistory>::with_config(config)?;
    rl.set_helper(Some(MyHelper {
        completer: rustyline::completion::FilenameCompleter::new(),
        highlighter: MatchingBracketHighlighter::new(),
        hinter: rustyline::hint::HistoryHinter::new(),
        validator: rustyline::validate::MatchingBracketValidator::new(),
    }));

    loop {
        let shell_cfg = lua::config::init_shell().unwrap_or_else(|_| {
            lua::config::ShellConfig::default()
        });

        let display_prompt = lua::config::parse_prompt(&shell_cfg.raw_prompt_template);

        match rl.readline(&display_prompt) {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() { continue; }

                let mut parts = input.split_whitespace();
                let raw_cmd = parts.next().unwrap_or("");
                let resolved_cmd = shell_cfg.aliases.get(raw_cmd).map(|s| s.as_str()).unwrap_or(raw_cmd);
                let args: Vec<&str> = parts.collect();

                let result: std::result::Result<(), String> = match resolved_cmd {
                    "exit" | "quit" => break,
                    "ls" => { command::ls::ls(); Ok(()) },
                    "clear" | "cls" => {
                        command::cls::clear_screen();
                        Ok(())
                    },
                    "cd" => {
                        let new_dir = args.first().cloned().unwrap_or(".");
                        env::set_current_dir(new_dir).map_err(|e| e.to_string())
                    },
                    "source" => {
                        if let Some(f) = args.first() { handle_source(f) } 
                        else { Err("Missing file argument".into()) }
                    },
                    "export" => {
                        if let Some(arg) = args.first() {
                            let kv: Vec<&str> = arg.splitn(2, '=').collect();
                            if kv.len() == 2 {
                                unsafe { env::set_var(kv[0], kv[1]); }
                                Ok(())
                            } else { Err("Usage: export KEY=VALUE".into()) }
                        } else { Err("Usage: export KEY=VALUE".into()) }
                    },
                    "SHELL" => {
                        println!("NaShell:\t\t\t\t\t/usr/local/bin/shell");
                        Ok(())
                    },
                    _ => {
                        let final_input = if raw_cmd != resolved_cmd {
                            input.replacen(raw_cmd, resolved_cmd, 1)
                        } else {
                            input.to_string()
                        };
                        
                        Command::new("bash").arg("-c").arg(&final_input).status()
                            .map(|_| ()).map_err(|e| e.to_string())
                    }
                };

                if let Err(e) = result {
                    eprintln!("{}", e.red());
                }
            },
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(e) => {
                eprintln!("{}", e.to_string().red());
                break;
            }
        }
    }
    Ok(())
}
