use std::{fs, path::{Path, PathBuf}};
use clap::Parser; 
use strum::Display;
use owo_colors::OwoColorize;
use tabled::{Table, Tabled, settings::{Color, Style, object::{Columns, Rows}}};
use chrono::{DateTime, Utc}; // Added import
use serde::Serialize;

#[derive(Debug, Display, Serialize)]
pub enum EntryType {
    File,
    Dir,
}
#[derive(Debug, Tabled, Serialize)]
pub struct FileEntry {
    #[tabled(rename = "Name")]
    name:String,
    #[tabled(rename = "Type")]
    e_type:EntryType,
    #[tabled(rename = "Size")]
    len_byte:u128, 
    modified:String,
}
#[derive(Debug, Parser)]
#[command(version, about, long_about = "Better Bash")]
pub struct Cli {
    /// The path to the file or directory
    #[arg(value_name = "PATH")]
    path: Option<PathBuf>,
    #[arg(short, long)]
    json: bool,
}

pub fn ls() {
    let cli = Cli::parse(); 
    let path = cli.path.unwrap_or(PathBuf::from("."));

    if path.exists() {
        let gfile = get_files(&path);
        let mut table = Table::new(gfile);
        table.with(Style::rounded()); 
        
        table.modify(Columns::first(), Color::FG_BRIGHT_CYAN);
        table.modify(Columns::one(2), Color::FG_BRIGHT_MAGENTA); // Fixed one() and index
        table.modify(Columns::one(3), Color::FG_BRIGHT_YELLOW); // Fixed one() and index
        table.modify(Rows::first(), Color::FG_BRIGHT_GREEN);
        
        if cli.json {
            let file = get_files(&path);
            println!("{}", serde_json::to_string(&file).unwrap_or("Can't Make Json Format".to_string())); 
        }else {
            println!("{}", table);
        }
    } else {
        println!("{}", "\n\tPath Does Not Exist\n".red());
    }
}

pub fn get_files(path:&Path) -> Vec<FileEntry> {
    let mut data = Vec::default();
    if let Ok(entry) = fs::read_dir(path) {
        for ent in entry.flatten() {
            map_data(ent, &mut data);
        }
    }
    data
}

pub fn map_data(file: fs::DirEntry, data: &mut Vec<FileEntry>) {
    if let Ok(pat) = fs::metadata(file.path()) {
        data.push(FileEntry { 
            name: file
                .file_name()
                .to_string_lossy()
                .into_owned(), 
            e_type: if pat.is_dir() {
                EntryType::Dir
            } else {
                EntryType::File
            },
            len_byte: pat.len() as u128,
            modified: if let Ok(time) = pat.modified() {
               let dt: DateTime<Utc> = time.into(); // Changed name to dt
               format!("{}", dt.format("%a %b %e %Y"))
            } else {
                String::default()
            },
        });
    }
}
