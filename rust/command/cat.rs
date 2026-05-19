use std::fs;

pub fn cat(filename: &str) -> std::io::Result<()> {
    let content = fs::read_to_string(filename)?;
    println!("{}", content);
    Ok(())
}
