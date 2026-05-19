use std::fs;

pub fn rm(path: &str) -> std::io::Result<()> {
    if std::path::Path::new(path).is_dir() {
        fs::remove_dir_all(path) // Recursive remove for dirs
    } else {
        fs::remove_file(path)    // Normal remove for files
    }
}
