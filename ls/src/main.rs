use std::fs;

fn main() {
    let paths = fs::read_dir(".").unwrap();

    for path in paths {
        let display_path = path.unwrap().path().display().to_string().replace("./", "");
        println!("{}", display_path);
    }
}
