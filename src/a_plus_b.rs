// Implements http://rosettacode.org/wiki/A%2BB
use std::io;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input).unwrap();
    let words = input.split_whitespace().take(2)
                            .map(|i| i.parse::<i32>().ok())
                            .collect::<Vec<Option<i32>>>();

    let sum = words.iter().fold(0, |a, &b| {
            if let Some(x) =  b { a + x }
            else { panic!("Please enter 2 integers") }
        }
    );

    println!("{}", sum);
}
