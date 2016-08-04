extern crate rand;

use std::io::stdin;
use rand::{Rng, thread_rng};

const LOWEST: isize = 1;
const HIGHEST: isize = 100;

fn main() {
    let mut rng = thread_rng();

    loop {
        let number: isize = rng.gen_range(LOWEST, HIGHEST + 1);
        let mut num_guesses = 0;

        println!("I have chosen my number between {} and {}. You know what to do",
                 LOWEST,
                 HIGHEST);

        loop {
            num_guesses += 1;

            let mut line = String::new();
            let res = stdin().read_line(&mut line);
            let input: Option<isize> = res.ok().map_or(None, |_| line.trim().parse().ok());

            match input {
                None => println!("numbers only, please"),
                Some(n) if n == number => {
                    println!("you got it in {} tries!", num_guesses);
                    break;
                }
                Some(n) if n < number => println!("too low!"),
                Some(n) if n > number => println!("too high!"),
                Some(_) => println!("something went wrong"),
            }
        }
    }
}
