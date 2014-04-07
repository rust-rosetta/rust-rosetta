// Implements http://rosettacode.org/wiki/Letter_frequency

extern crate collections;

use std::io::fs::File;
use std::io::BufferedReader;
use std::os;

fn main() {
    let filename = match os::args().len() {
        1 => fail!("You must enter a filename to read line by line"),
        _ => os::args()[1]
    };

    let file = File::open(&Path::new(filename));
    let mut reader = BufferedReader::new(file);

    let mut s : collections::HashMap<char,uint> = collections::HashMap::new();

    for c in reader.chars() {
        let letter = c.unwrap();
        let counter = match s.contains_key(&letter) {
            true => s.get(&letter) + 1,
            false => 1,
        };
        s.insert(letter, counter);
    }

    println!("{}", s);
}
