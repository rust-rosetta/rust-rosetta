// Implements http://rosettacode.org/wiki/A%2BB

use std::io::stdio::stdin;
use std::from_str::from_str;
use std::io::BufferedReader;

fn main() {
    let input = BufferedReader::new(stdin()).read_line().unwrap();
    let mut words = input.words();

    let sum = match (words.next().and_then(from_str::<int>), words.next().and_then(from_str)) {
        (Some(a), Some(b)) => a + b,
        _                  => fail!("Please enter 2 integers")
    };

    println!("{:i}", sum);     
}