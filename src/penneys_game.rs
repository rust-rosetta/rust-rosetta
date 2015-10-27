// http://rosettacode.org/wiki/Penney's_game
// H = true, T = false

extern crate rand;

use std::io::{stdout, stdin, Write};
use rand::{Rng, thread_rng};

fn toss_coin(print:bool) -> char {
    let c : char;
    if thread_rng().gen::<bool>() {c = 'H'} else {c = 'T'}
    if print {
        print!("{}",c);
        stdout().flush().ok().expect("Could not flush stdout");
        }
    c
}

fn gen_sequence(seed : Option<&str>) -> String {
    let mut seq = String::new();
    match seed {
        Some(s) => {
            let c0 = s.chars().next().unwrap();
            if c0 == 'H' {seq.push('T')
                } else {seq.push('H')}
            seq.push(c0);
            seq.push(s.chars().nth(1).unwrap());
        },
        None => {
            for _ in (0..3) {
                seq.push(toss_coin(false))
            }
        }
    }
    seq
}

fn read_sequence() -> String {
    let mut seq = String::new();
    'reading: loop {
        seq.clear();
        println!("Please, enter sequence of 3 coins: H (heads) or T (tails): ");
        stdin().read_line(&mut seq)
                   .ok().expect("failed to read line");
        seq = seq.trim().to_uppercase();
        if seq.len() != 3 { continue
        } else {
            for c in seq.chars() {
                match c {
                    'H' | 'T' => continue,
                    _ => {
                        println!("Please enter correct sequence!");
                        continue 'reading;
                    }
                };
            }
            return seq;
        }
    }
}

fn main() {
    let useq : String;
    let aiseq : String;
    if thread_rng().gen::<bool>() {
        println!("You choose first!");
        useq = read_sequence();
        println!("Your sequence: {}", useq);
        aiseq = gen_sequence( Some(&useq) );
        println!("My sequence: {}", aiseq);
    } else {
        println!("I choose first!");
        aiseq = gen_sequence(None);
        println!("My sequence: {}", aiseq);
        useq = read_sequence();
        println!("Your sequence: {}", useq);
    }
    println!("Tossing coins...");
    let mut coins = String::new();
    for _ in (0..2) { //toss first 2 coins
        coins.push(toss_coin(true));
        std::thread::sleep_ms(500);
    }
    loop {
        coins.push(toss_coin(true));
        std::thread::sleep_ms(500);
        if coins.contains(&useq) {
            println!("\nYou win!");
            break;
        }
        if coins.contains(&aiseq) {
            println!("\nI win!");
            break;
        }
    }
}
