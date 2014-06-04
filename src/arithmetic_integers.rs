// Implements http://rosettacode.org/wiki/Arithmetic/Integer
// not_tested

#![allow(unused_imports)]
use std::io::stdio::stdin;
use std::io::BufferedReader;

fn main() {
    let input = stdin().read_line().unwrap();
    let words = input.as_slice().words().take(2)
                            .map(from_str)
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
