// Implements http://rosettacode.org/wiki/Concurrent_computing
extern crate rand;

use std::io::timer::sleep;
use rand::random;

fn main() {
    let strings = ~[~"Enjoy", ~"Rosetta", ~"Code"];

    for s in strings.move_iter(){
        spawn(proc() {
            // We use a random u8 (so an integer from 0 to 255)
            sleep(random::<u8>() as u64);
            println!("{}", s);
        });
    }
}
