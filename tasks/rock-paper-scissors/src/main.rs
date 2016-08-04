extern crate rand;

use std::io;
use rand::{Rng, thread_rng};
use Choice::*;

#[derive(PartialEq, Clone, Copy)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn print(&self) -> &str {
        match *self {
            Rock => "Rock",
            Paper => "Paper",
            Scissors => "Scissors",
        }
    }

    fn rand(&mut self) {
        match thread_rng().gen_range(0, 2) {
            0 => *self = Rock,
            1 => *self = Paper,
            2 | _ => *self = Scissors,
        }
    }
}

fn beats(c1: Choice, c2: Choice) -> bool {
    (c1 == Rock) && (c2 == Scissors) || (c1 == Scissors && c2 == Paper) ||
    (c1 == Paper && c2 == Rock)
}

fn ai_move(v: [usize; 3]) -> Choice {
    // weighted random choice
    let sum = thread_rng().gen_range(0, v[0] + v[1] + v[2]);
    if sum < v[0] {
        Paper
    } else if sum < v[0] + v[1] {
        Scissors
    } else {
        Rock
    }
}

fn main() {
    println!("Rock, paper, scissors!");
    let mut aichoice: Choice = Rock;
    aichoice.rand();
    let mut uchoice: Choice;
    let mut ucf: [usize; 3] = [0, 0, 0]; //user choice frequency
    let mut score: [usize; 2] = [0, 0];
    println!("Please input your move: r, p or s. Type q to quit");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        match input.to_lowercase().trim() {
            s if s.starts_with('r') => {
                uchoice = Rock;
                ucf[0] += 1
            }
            s if s.starts_with('p') => {
                uchoice = Paper;
                ucf[1] += 1
            }
            s if s.starts_with('s') => {
                uchoice = Scissors;
                ucf[2] += 1
            }
            s if s.starts_with('q') => break,
            _ => {
                println!("Please enter correct choice!");
                continue;
            }
        };
        println!("You chose {}, I chose {}.",
                 uchoice.print(),
                 aichoice.print());
        if beats(uchoice, aichoice) {
            score[0] += 1;
            println!("You win!");
        } else if uchoice == aichoice {
            println!("It's a tie!");
        } else {
            score[1] += 1;
            println!("I win!");
        }
        println!("-Score: You {}, Me {}", score[0], score[1]);

        aichoice = ai_move(ucf);
        // only after the 1st iteration ai knows the stats and can make
        // its weighted random move
    }
    println!("Thank you for the game!");
}
