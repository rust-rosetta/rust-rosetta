#![feature(slice_patterns)]

use std::io::stdin;

fn main() {
    let mut input = String::new();
    let _ = stdin().read_line(&mut input).unwrap();
    let words = input.split_whitespace()
        .take(2)
        .map(|s| s.parse().ok())
        .collect::<Vec<Option<i32>>>();

    let (a, b) = match &words[..] {
        &[Some(x), Some(y)] => (x, y),
        _ => panic!("Please enter 2 integers"),
    };

    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {}", a / b);
    println!("a % b = {}", a % b);
}
