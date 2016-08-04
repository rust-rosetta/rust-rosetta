extern crate rand;

use std::io::{stdout, stdin, Write};
use std::thread;
use std::time::Duration;
use rand::{Rng, thread_rng};

fn toss_coin(print: bool) -> char {
    let c: char;
    if thread_rng().gen::<bool>() {
        c = 'H'
    } else {
        c = 'T'
    }
    if print {
        print!("{}", c);
        stdout().flush().expect("Could not flush stdout");
    }
    c
}

fn gen_sequence(seed: Option<&str>) -> String {
    let mut seq = String::new();
    match seed {
        Some(s) => {
            let c0 = s.chars().next().unwrap();
            if c0 == 'H' {
                seq.push('T')
            } else {
                seq.push('H')
            }
            seq.push(c0);
            seq.push(s.chars().nth(1).unwrap());
        }
        None => {
            for _ in 0..3 {
                seq.push(toss_coin(false))
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
        if !(seq.chars().all(|c| c == 'H' || c == 'T') && seq.len() == 3 &&
             seq != used_seq.unwrap_or("")) {
            println!("Please enter correct sequence!");
            continue;
        }
        return seq;
    }
}

fn main() {
    println!("--Penney's game--");
    loop {
        let useq: String;
        let aiseq: String;
        if thread_rng().gen::<bool>() {
            println!("You choose first!");
            useq = read_sequence(None);
            println!("Your sequence: {}", useq);
            aiseq = gen_sequence(Some(&useq));
            println!("My sequence: {}", aiseq);
        } else {
            println!("I choose first!");
            aiseq = gen_sequence(None);
            println!("My sequence: {}", aiseq);
            useq = read_sequence(Some(&aiseq));
            println!("Your sequence: {}", useq);
        }
        println!("Tossing coins...");
        let mut coins = String::new();
        for _ in 0..2 {
            // toss first 2 coins
            coins.push(toss_coin(true));
            thread::sleep(Duration::from_millis(500));
        }
        loop {
            coins.push(toss_coin(true));
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

        println!(" Play again? Y to play, Q to exit.");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("failed to read line");
        match input.chars().next().unwrap() {
            'Y' | 'y' => continue,
            _ => break,
        }
    }
}
