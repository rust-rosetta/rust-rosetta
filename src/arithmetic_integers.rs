// Implements http://rosettacode.org/wiki/Arithmetic/Integer
#![cfg(not_tested)]

#![allow(unused_imports)]
use std::io::stdio::stdin;
use std::from_str::from_str;
use std::io::BufferedReader;

fn main() {
    let input = BufferedReader::new(stdin()).read_line().unwrap();
    let words = input.as_slice().words().take(2)
                            .map(from_str::<int>)
                            .collect::<Vec<Option<int>>>();

    let (a, b) = match words.as_slice() {
            [Some(x), Some(y)] => (x, y),
            _                  => fail!("Please enter 2 integers")
    };

    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {}", a / b);
    println!("a % b = {}", a % b);
}
