// Implements http://rosettacode.org/wiki/Guess_the_number
use std::rand::{task_rng, Rng};
use std::io::stdio::stdin;

fn main() {
    let mystery_number = task_rng().gen_range(0i, 10) + 1;
    println!("Guess my number between 1 and 10");

    let mut input = stdin();
    loop {
        let line = input.read_line().unwrap();
        match from_str::<int>(line.as_slice().trim()) {
            Some(guess) if guess == mystery_number => break,
            Some(_) => println!("Wrong! Try again!"),
            None => println!("Please enter an integer")
        }
    }

    // The loop ends only if the player wins
    println!("Well guessed!");
}
