// Implements http://rosettacode.org/wiki/Read_a_file_line_by_line
#![feature(old_io)]
#![feature(old_path)]

use std::old_io::fs::File;
use std::old_io::BufferedReader;
use std::env::args;
use std::borrow::ToOwned;

fn main() {
    let filename = {
        if let Some(o_s) = args().nth(1) {
            o_s.to_owned()
        } else {
            panic!("You must enter a filename to read line by line")
        }
    };

    let file = File::open(&Path::new(&filename[..]));
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
