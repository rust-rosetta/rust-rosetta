// Implements http://rosettacode.org/wiki/Concurrent_computing
use std::io::timer::sleep;
use std::rand::random;

fn main() {
    let strings = ~[~"Enjoy", ~"Rosetta", ~"Code"];

    for s in strings.move_iter(){
        do spawn {
            sleep(random::<u8>() as u64);
            println!("{:s}", s);
        }
    }
}