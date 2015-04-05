// Implements http://rosettacode.org/wiki/Concurrent_computing
extern crate rand;
use std::thread::sleep_ms;
use rand::random;
use std::thread::spawn;

fn main() {
    let strings = vec!["Enjoy", "Rosetta", "Code"];

    for s in strings.into_iter(){
        spawn(move || -> () {
            // We use a random u8 (so an integer from 0 to 255)
            sleep_ms(random::<u8>() as u32);
            println!("{}", s);
        });
    }
}
