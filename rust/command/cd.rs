use std::env;
use std::path::Path;

pub fn cd(args: Vec<&str>) -> Result<(), String> {
    // 1. Get destination, default to home "~"
    let dest = args.get(0).copied().unwrap_or("~");

    // 2. Resolve "~" to the actual home path
    let path_to_set = if dest == "~" {
        env::var("HOME")
            .or_else(|_| env::var("USERPROFILE"))
            .map_err(|_| "Could not find home directory".to_string())?
    } else {
        dest.to_string()
    };

    // 3. Change directory
    env::set_current_dir(Path::new(&path_to_set))
        .map_err(|e| format!("Directory not found: {}", e))
}
