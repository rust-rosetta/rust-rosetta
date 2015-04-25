// Implements http://rosettacode.org/wiki/A%2BB
#![feature(slice_patterns)]
use std::io;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input).unwrap();
    let words = input.split_whitespace().take(2)
                            .map(|i| i.parse::<i32>().ok())
                            .collect::<Vec<Option<i32>>>();

    let sum = match &words[..] {
        [Some(x), Some(y)] => x + y,
            _ => panic!("Please enter 2 integers")
    };

    println!("{}", sum);
}
