// Implements http://rosettacode.org/wiki/Arithmetic/Integer

use std::io::stdio::stdin;
use std::from_str::from_str;
use std::io::BufferedReader;

fn main() {
    let input = BufferedReader::new(stdin()).read_line().unwrap();
    let mut words = input.words();
    let a = match words.next().and_then(from_str::<int>) {
            Some(_a) => _a,
            _        => fail!("Please enter 2 integers")
    };
    let b = match words.next().and_then(from_str::<int>) {
            Some(_b) => _b,
            _        => fail!("Please enter 2 integers")
    };

    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {}", a / b);
    println!("a % b = {}", a % b);
}
