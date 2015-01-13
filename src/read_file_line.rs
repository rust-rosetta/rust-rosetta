// Implements http://rosettacode.org/wiki/Read_a_file_line_by_line
#![allow(unstable)]
use std::io::fs::File;
use std::io::BufferedReader;
use std::os::args;

fn main() {
    let filename = match args().len() {
        1 => panic!("You must enter a filename to read line by line"),
        _ => args()[1].clone()
    };

    let file = File::open(&Path::new(&filename[]));
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
