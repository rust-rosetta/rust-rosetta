// Implements http://rosettacode.org/wiki/Read_a_file_line_by_line
#![allow(dead_code)] // not_tested

use std::io::fs::File;
use std::io::BufferedReader;
use std::os::args;

fn main() {
    let filename = match args().len() {
        1 => fail!("You must enter a filename to read line by line"),
        _ => args()[1].clone()
    };

    let file = File::open(&Path::new(filename.as_slice()));
    let mut reader = BufferedReader::new(file);

    for line in reader.lines() {
        // Handle any errors that may arise
        match line {
            Ok(ln) => print!("{}", ln),
            Err(error) => print!("{}", error.desc)
        }
    }
    println!("");
}
