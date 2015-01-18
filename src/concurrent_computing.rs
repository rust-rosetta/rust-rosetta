// Implements http://rosettacode.org/wiki/Concurrent_computing
#![allow(unstable)]
use std::io::timer::sleep;
use std::rand::random;
use std::time::duration::Duration;
use std::thread::Thread;

fn main() {
    let strings = vec!["Enjoy", "Rosetta", "Code"];

    for s in strings.into_iter(){
        Thread::spawn(move || -> () {
            // We use a random u8 (so an integer from 0 to 255)
            sleep(Duration::milliseconds(random::<u8>() as i64));
            println!("{}", s);
        });
    }
}
