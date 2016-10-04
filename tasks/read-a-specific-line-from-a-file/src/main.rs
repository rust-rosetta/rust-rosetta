use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env::args;
use std::borrow::ToOwned;

fn main() {
    let mut args = args();

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
                .parse::<usize>()
                .expect("You must enter an integer as the line number")
        } else {
            panic!("You must enter a filename to read line by line")
        }
    };

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    match reader.lines().skip(line_number - 1).next() {
        None => panic!("No such line (file is too short)"),
        Some(result) => {
            match result {
                // Handle any errors that may arise
                Ok(ln) => print!("{}", ln),
                Err(error) => print!("{}", error),
            }
        }
    }
}
