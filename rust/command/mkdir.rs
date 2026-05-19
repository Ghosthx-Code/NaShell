pub fn dir(name: &str) -> std::io::Result<()> {
    std::fs::create_dir_all(name)?; 
    Ok(())
}
