// Implements http://rosettacode.org/wiki/Guess_the_number
#![cfg(not_tested)]

use std::rand::{task_rng, Rng};
use std::io::stdio::stdin;
use std::io::BufferedReader;

fn main() {
    let mystery_number = task_rng().gen_range(0, 10) + 1;
    println!("Guess my number between 1 and 10");

    let mut input = BufferedReader::new(stdin());
    loop {
        let line = input.read_line().unwrap();
        match from_str::<int>(line.as_slice().trim()) {
            Some(number) if (number == mystery_number) => break,
            Some(_) => println!("Wrong! Try again!"),
            None    => println!("Please enter an integer")
        }
    }

    // The loop ends only if the player wins
    println!("Well guessed!");
}
