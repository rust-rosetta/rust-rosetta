// Implements http://rosettacode.org/wiki/Unix/ls#Rust
// Works only with correct paths or no arguments at all

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // ignoring all arguments except the 1st
    match env::args().nth(1) { // check if the program received an argument
        Some(path) => { print_files(Path::new(&path)); }
        _ => { print_files( &env::current_dir().unwrap() ); }
    // note that current_dir value might be invalid, so it's a Result
    }
}

fn print_files(path:&Path) {
    let mut entries: Vec<String> = fs::read_dir(path).unwrap()
        .map(|x|
            x.unwrap().path() // DirEntry to PathBuf
            .file_name().unwrap() // trim to file name
            .to_os_string()
            .into_string().unwrap()
        ).collect(); // collecting Vec of file names
    entries.sort();
    for x in entries {println!("{}", x); }
}
