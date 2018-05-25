extern crate rand;

use std::io;

use rand::distributions::{Standard, Uniform};
use rand::prelude::*;

use Choice::*;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Distribution<Choice> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Choice {
        // Use `Uniform` rather than `gen_range`'s `Uniform::sample_single` for speed
        let range = Uniform::new(0, 3);
        match rng.sample(range) {
            0 => Rock,
            1 => Paper,
            2 => Scissors,
            _ => unreachable!(), // `_ | 2` would remove the check
        }
    }
}

fn beats(c1: Choice, c2: Choice) -> bool {
    (c1 == Rock && c2 == Scissors) || (c1 == Scissors && c2 == Paper) || (c1 == Paper && c2 == Rock)
}

fn ai_move<R: Rng>(rng: &mut R, v: [usize; 3]) -> Choice {
    // weighted random choice, a dynamic version of `rand::distributions::WeightedChoice`
    let rand = rng.gen_range(0, v[0] + v[1] + v[2]);
    if rand < v[0] {
        Paper
    } else if rand < v[0] + v[1] {
        Scissors
    } else {
        Rock
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    println!("Rock, paper, scissors!");
    let mut ai_choice: Choice = rng.gen();
    let mut ucf = [0, 0, 0]; // user choice frequency
    let mut score = [0, 0];

    loop {
        println!("Please input your move: 'r', 'p' or 's'. Type 'q' to quit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        // trim leading whitespace, get first lowercase character
        let u_choice = match input
            .trim_left()
            .chars()
            .next()
            .and_then(|c| c.to_lowercase().next())
        {
            Some('r') => {
                ucf[0] += 1;
                Rock
            }
            Some('p') => {
                ucf[1] += 1;
                Paper
            }
            Some('s') => {
                ucf[2] += 1;
                Scissors
            }
            Some('q') => break,
            _ => {
                println!("Please enter a correct choice!");
                continue;
            }
        };
        println!("You chose {:?}, I chose {:?}.", u_choice, ai_choice);
        if beats(u_choice, ai_choice) {
            score[0] += 1;
            println!("You win!");
        } else if u_choice == ai_choice {
            println!("It's a tie!");
        } else {
            score[1] += 1;
            println!("I win!");
        }
        println!("-Score: You {}, Me {}", score[0], score[1]);

        // only after the 1st iteration the AI knows the stats and can make
        // its weighted random move
        ai_choice = ai_move(&mut rng, ucf);
    }
    println!("Thank you for the game!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_victory() {
        assert!(beats(Scissors, Paper));
        assert!(beats(Rock, Scissors));
        assert!(beats(Paper, Rock));
    }

    #[test]
    fn rand_choice() {
        let mut rng = thread_rng();
        for _ in 0..4 {
            rng.gen::<Choice>();
        }
    }
}
