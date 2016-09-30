use std::fs::File;
use std::io::{BufReader, BufRead};
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

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        // Handle any errors that may arise
        match line {
            Ok(ln) => print!("{}", ln),
            Err(error) => print!("{}", error),
        }
    }
    println!("");
}
