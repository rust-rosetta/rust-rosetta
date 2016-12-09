use std::fs;

fn main() {
    fs::remove_file("input.txt").unwrap();
    fs::remove_file("/input.txt").unwrap();
    fs::remove_dir_all("docs").unwrap();
    fs::remove_dir_all("/docs").unwrap();
}
