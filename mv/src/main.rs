use std::fs;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let original_file = &args[1];
    let new_file = &args[2];

    // Rename original file to a new
    fs::rename(original_file, new_file)?;

    Ok(())
}
