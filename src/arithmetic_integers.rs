// Implements http://rosettacode.org/wiki/Arithmetic/Integer
#![allow(unstable)]
use std::io::stdin;

fn main() {
    let input = stdin().read_line().unwrap();
    let words = input.words().take(2)
                                        .map(|s| s.parse())
                                        .collect::<Vec<Option<i32>>>();

    let (a, b) = match &words[] {
            [Some(x), Some(y)] => (x, y),
            _ => panic!("Please enter 2 integers")
    };

    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {}", a / b);
    println!("a % b = {}", a % b);
}
