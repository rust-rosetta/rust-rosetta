// Implements http://rosettacode.org/wiki/A%2BB
#![feature(old_io)]
#![feature(str_words)]
use std::old_io::stdio;

fn main() {
    let input = stdio::stdin().read_line().unwrap();
    let words = input.words().take(2)
                            .map(|i| i.parse::<i32>().ok())
                            .collect::<Vec<Option<i32>>>();

    let sum = match &words[..] {
        [Some(x), Some(y)] => x + y,
            _ => panic!("Please enter 2 integers")
    };

    println!("{}", sum);
}
