// Implements http://rosettacode.org/wiki/Letter_frequency

extern crate collections;

#[cfg(not(test))]
use std::io::fs::File;
#[cfg(not(test))]
use std::io::BufferedReader;
#[cfg(not(test))]
use std::os;

#[cfg(not(test))]
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
