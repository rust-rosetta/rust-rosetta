extern crate rand;

use std::io::stdin;

use rand::{thread_rng, Rng};

fn main() {
    let mystery_number = thread_rng().gen_range(0u8, 10) + 1;
    println!("Guess my number between 1 and 10");

    let input = stdin();
    loop {
        let mut line = String::new();
        let _ = input.read_line(&mut line).unwrap();
        match line.trim().parse::<u8>() {
            Ok(guess) if guess == mystery_number => break,
            Ok(_) => println!("Wrong! Try again!"),
            Err(_) => println!("Please enter an integer"),
        }
    }

    // The loop ends only if the player wins
    println!("Well guessed!");
}
