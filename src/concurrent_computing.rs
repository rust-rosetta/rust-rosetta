// Implements http://rosettacode.org/wiki/Concurrent_computing

#![feature(std_misc)]
#![feature(old_io)]

extern crate rand;
use std::old_io::timer::sleep;
use rand::random;
use std::time::duration::Duration;
use std::thread::spawn;

fn main() {
    let strings = vec!["Enjoy", "Rosetta", "Code"];

    for s in strings.into_iter(){
        spawn(move || -> () {
            // We use a random u8 (so an integer from 0 to 255)
            sleep(Duration::milliseconds(random::<u8>() as i64));
            println!("{}", s);
        });
    }
}
