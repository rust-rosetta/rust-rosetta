// Implements http://rosettacode.org/wiki/Guess_the_number

#![feature(old_io)]
extern crate rand;
use rand::{thread_rng, Rng};
use std::old_io::stdio::stdin;

fn main() {
    let mystery_number = thread_rng().gen_range(0u8, 10) + 1;
    println!("Guess my number between 1 and 10");

    let mut input = stdin();
    loop {
        let line = input.read_line().unwrap();
        match line.trim().parse::<u8>() {
            Ok(guess) if guess == mystery_number => break,
            Ok(_) => println!("Wrong! Try again!"),
            Err(_) => println!("Please enter an integer")
        }
    }

    // The loop ends only if the player wins
    println!("Well guessed!");
}
