extern crate rand;

use std::io::{stdin, stdout, Write};
use std::thread;
use std::time::Duration;

use rand::Rng;

fn toss_coin<R: Rng>(rng: &mut R, print: bool) -> char {
    let c = if rng.gen() { 'H' } else { 'T' };
    if print {
        print!("{}", c);
        stdout().flush().expect("Could not flush stdout");
    }
    c
}

fn gen_sequence<R: Rng>(rng: &mut R, seed: Option<&str>) -> String {
    let mut seq = String::new();
    match seed {
        Some(s) => {
            let mut iter = s.chars();
            let c0 = iter.next().unwrap();
            let next = if c0 == 'H' { 'T' } else { 'H' };
            seq.push(next);
            seq.push(c0);
            seq.push(iter.next().unwrap());
        }
        None => {
            for _ in 0..3 {
                seq.push(toss_coin(rng, false))
            }
        }
    }
    seq
}

fn read_sequence(used_seq: Option<&str>) -> String {
    let mut seq = String::new();
    loop {
        seq.clear();
        println!("Please, enter sequence of 3 coins: H (heads) or T (tails): ");
        stdin().read_line(&mut seq).expect("failed to read line");
        seq = seq.trim().to_uppercase();
        // do the cheapest test first
        if seq.len() == 3
            && seq.chars().all(|c| c == 'H' || c == 'T')
            && seq != used_seq.unwrap_or("")
        {
            return seq;
        }

        println!("Please enter correct sequence!");
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    println!("--Penney's game--");
    loop {
        let useq: String;
        let aiseq: String;
        if rng.gen::<bool>() {
            println!("You choose first!");
            useq = read_sequence(None);
            println!("Your sequence: {}", useq);
            aiseq = gen_sequence(&mut rng, Some(&useq));
            println!("My sequence: {}", aiseq);
        } else {
            println!("I choose first!");
            aiseq = gen_sequence(&mut rng, None);
            println!("My sequence: {}", aiseq);
            useq = read_sequence(Some(&aiseq));
            println!("Your sequence: {}", useq);
        }
        println!("Tossing coins...");
        let mut coins = String::new();
        for _ in 0..2 {
            // toss first 2 coins
            coins.push(toss_coin(&mut rng, true));
            thread::sleep(Duration::from_millis(500));
        }
        loop {
            coins.push(toss_coin(&mut rng, true));
            thread::sleep(Duration::from_millis(500));
            if coins.contains(&useq) {
                println!("\nYou win!");
                break;
            }
            if coins.contains(&aiseq) {
                println!("\nI win!");
                break;
            }
        }

        println!(" Play again? 'Y' to play, 'Q' to exit.");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("failed to read line");
        match input.trim_start().chars().next().unwrap() {
            'Y' | 'y' => continue,
            _ => break,
        }
    }
}
