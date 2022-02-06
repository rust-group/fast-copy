use std::fs;

pub fn copy(file: &str, to_file: &str) {
    fs::copy(file, to_file).unwrap();
    println!("copy finished");
}
