//! Works only with correct paths or no arguments at all

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // ignoring all arguments except the 1st
    match env::args().nth(1) { // check if the program received an argument
        Some(path) => {
            print_files(Path::new(&path));
        }
        _ => {
            print_files(&env::current_dir().unwrap());
        }
        // note that current_dir value might be invalid, so it's a Result
    }
}

fn print_files(path: &Path) {
    let mut entries: Vec<_> = fs::read_dir(path)
        .unwrap()
        .map(|x| x.unwrap().file_name())
        .collect();
    entries.sort();
    for x in entries {
        println!("{}", x.to_string_lossy());
    }
}
