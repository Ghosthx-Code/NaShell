pub fn touch(name: &str) -> std::io::Result<()> {
    std::fs::File::create(name)?; // The '?' returns the error if it fails
    Ok(())
}

