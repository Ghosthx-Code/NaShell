use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use walkdir::WalkDir;

pub fn count_lines_in_file(path: &Path) -> std::io::Result<usize> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().count())
}

pub fn line() {
    let target_dir = ".";
    let mut total_lines = 0;
    let mut file_count = 0;

    println!("Counting Lines In Dir: {}", target_dir);

    for entry in WalkDir::new(target_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() {
            match count_lines_in_file(path) {
                Ok(lines) => {
                    total_lines += lines;
                    file_count += 1;
                    println!("{}: {} Lines", path.display(), lines);
                }
                Err(e) => eprintln!("Error Reading {} : {}", path.display(), e),
            }
        }
    }

    println!("\nSummary");
    println!("Total Files: {}", file_count);
    println!("Total Lines: {}", total_lines);
}

