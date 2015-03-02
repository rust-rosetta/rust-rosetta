// Implements http://rosettacode.org/wiki/Read_a_specific_line_from_a_file
#![feature(old_io)]
#![feature(old_path)]

use std::old_io::fs::File;
use std::old_io::BufferedReader;
use std::env::args;
use std::borrow::ToOwned;

fn main() {
    let mut args =  args();

    let filename = {
        if let Some(o_s) = args.nth(1) {
            o_s.to_owned()
        } else {
            panic!("You must enter a filename to read line by line")
        }
    };

    let line_number = {
        if let Some(o_s) = args.next() {
            o_s.to_owned()
                .parse::<usize>().ok()
                .expect("You must enter an integer as the line number")
        } else {
            panic!("You must enter a filename to read line by line")
        }
    };

    let file = File::open(&Path::new(&filename[..]));
    let mut reader = BufferedReader::new(file);

    match reader.lines().skip(line_number-1).next() {
        None => panic!("No such line (file is too short)"),
        Some(result) => match result {
            // Handle any errors that may arise
            Ok(ln) => print!("{}", ln),
            Err(error) => print!("{}", error)
        }
    }
}

