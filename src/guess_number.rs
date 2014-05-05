// Implements http://rosettacode.org/wiki/Guess_the_number
extern crate rand;

#[cfg(not(test))]
use rand::{task_rng, Rng};
#[cfg(not(test))]
use std::io::stdio::stdin;
#[cfg(not(test))]
use std::io::BufferedReader;

#[cfg(not(test))]
fn main() {
    let mystery_number = task_rng().gen_range(0, 10) + 1;
    println!("Guess my number between 1 and 10");

    let mut input = BufferedReader::new(stdin());
    loop {
        let line = input.read_line().unwrap();
        match from_str::<int>(line.trim()) {
            Some(number) if (number == mystery_number) => break,
            Some(_) => println!("Wrong! Try again!"),
            None    => println!("Please enter an integer")
        }
    }

    // The loop ends only if the player wins
    println!("Well guessed!");
}
