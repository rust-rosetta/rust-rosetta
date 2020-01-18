//! Works only with correct paths or no arguments at all

use std::path::Path;
use std::{env, fs};

fn main() -> Result<(), std::io::Error> {
    // ignoring all arguments except the 1st
    if let Some(path) = env::args().nth(1) {
        // check if the program received an argument
        print_files(Path::new(&path))?;
    } else {
        // note that current_dir value might be invalid, so it's a Result
        print_files(&env::current_dir()?)?;
    };
    Ok(())
}

fn print_files(path: &Path) -> std::io::Result<()> {
    // flatten entries to avoid calling .unwrap()
    let mut entries: Vec<_> = fs::read_dir(path)?
        .flat_map(|res| res)
        .map(|f| f.file_name())
        .collect();
    // read_dir does not guarantee order
    entries.sort();
    entries.iter().for_each(|f| {
        println!("{}", f.to_string_lossy());
    });
    Ok(())
}
