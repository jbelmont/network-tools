use std::fs;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let original_file = &args[1];
    let copied_file = &args[2];

    // Copy the contents of original_file into copied_file
    fs::copy(original_file, copied_file)?;

    Ok(())
}
