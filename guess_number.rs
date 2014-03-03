// Implements http://rosettacode.org/wiki/Guess_the_number
use std::rand::{task_rng, Rng};
use std::io::stdio::stdin;
use std::io::BufferedReader;

fn main() {
    let mysteryNumber: int = task_rng().gen_range(0, 10)+1;
    println!("Guess my number between 1 and 10");
    let mut goodGuess = false;

    while !goodGuess {
        let userInput = BufferedReader::new(stdin()).read_line().unwrap();
        let userNumber = match from_str::<int>(userInput.slice_to(userInput.len() - 1)) {
            Some(number) => number,
            _            => fail!("Please enter an integer")
        };
        goodGuess = userNumber == mysteryNumber;
    }
    println!("Well guessed!");
}

