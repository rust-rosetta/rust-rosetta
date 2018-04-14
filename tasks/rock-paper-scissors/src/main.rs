extern crate rand;
#[macro_use]
extern crate rand_derive;

use rand::Rng;
use std::io;
use Choice::*;

#[derive(PartialEq, Clone, Copy, Rand, Debug)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
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
        let u_choice = match input.to_lowercase().trim() {
            s if s.starts_with('r') => {
                ucf[0] += 1;
                Rock
            }
            s if s.starts_with('p') => {
                ucf[1] += 1;
                Paper
            }
            s if s.starts_with('s') => {
                ucf[2] += 1;
                Scissors
            }
            s if s.starts_with('q') => break,
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

#[test]
fn test_victory() {
    assert!(beats(Scissors, Paper));
    assert!(beats(Rock, Scissors));
    assert!(beats(Paper, Rock));
}
