// Implements http://rosettacode.org/wiki/A%2BB
#![allow(unstable)]
use std::io::stdio;

fn main() {
    let input = stdio::stdin().read_line().unwrap();
    let words = input.words().take(2)
                            .map(|i| i.parse::<i32>())
                            .collect::<Vec<Option<i32>>>();

    let sum = match &words[] {
        [Some(x), Some(y)] => x + y,
            _ => panic!("Please enter 2 integers")
    };

    println!("{}", sum);
}
