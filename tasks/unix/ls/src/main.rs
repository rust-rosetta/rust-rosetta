//! Works only with correct paths or no arguments at all

use std::env;
use std::fs;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    // ignoring all arguments except the 1st
    match env::args().nth(1) {
        // check if the program received an argument
        Some(path) => {
            print_files(Path::new(&path))?;
        }
        None => {
            print_files(&env::current_dir()?)?;
        } // note that current_dir value might be invalid, so it's a Result
    };
    Ok(())
}

fn print_files(path: &Path) -> std::io::Result<()> {
    let mut entries: Vec<_> = fs::read_dir(path)?
        .map(|x| x.unwrap().file_name())
        .collect();
    entries.sort();
    for x in entries {
        println!("{}", x.to_string_lossy());
    }
    Ok(())
}
