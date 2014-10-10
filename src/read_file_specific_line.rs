// Implements http://rosettacode.org/wiki/Read_a_specific_line_from_a_file

use std::io::fs::File;
use std::io::BufferedReader;
use std::os::args;

fn main() {
    match args().len() {
        2 => fail!("You must enter a filename to read line by line, and a line number"),
        1 => fail!("You must enter a line number"),
        _ => {}
    }
    let filename = args()[1].clone();
    let line_number: uint = from_str(args()[2].as_slice()).expect("You must enter an integer as the line number");

    let file = File::open(&Path::new(filename.as_slice()));
    let mut reader = BufferedReader::new(file);

    match reader.lines().skip(line_number-1).next() {
        None => fail!("No such line (file is too short)"),
        Some(result) => match result {
            // Handle any errors that may arise
            Ok(ln) => print!("{}", ln),
            Err(error) => print!("{}", error.desc)
        }
    }
}

