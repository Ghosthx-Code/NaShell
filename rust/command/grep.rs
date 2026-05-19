use std::fs::File;
use std::io::{self, BufRead};
use owo_colors::OwoColorize;

pub fn grep(pattern: &str, filename: &str) {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}: {}", "Error".red(), e);
            return;
        }
    };

    let reader = io::BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        if let Ok(content) = line {
            if content.contains(pattern) {
                // Highlight the match and show line number
                let highlighted = content.replace(pattern, &pattern.red().bold().to_string());
                println!("{}: {}", (index + 1).yellow(), highlighted);
            }
        }
    }
}
