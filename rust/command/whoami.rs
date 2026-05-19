use std::env;
use owo_colors::OwoColorize;

pub fn whoami() {
    let user = env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .unwrap_or_else(|_| "unknown".to_string());
    println!("{}", user.bright_yellow().bold());
}
