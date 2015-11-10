// http://rosettacode.org/wiki/Concurrent_computing
extern crate rand;

use std::thread;
use std::time::Duration;

use rand::random;

fn main() {
    let strings = vec!["Enjoy", "Rosetta", "Code"];

    for s in strings.into_iter(){
        thread::spawn(move || {
            // We use a random u8 (so an integer from 0 to 255)
            thread::sleep(Duration::from_millis(random::<u8>() as u64));
            println!("{}", s);
        });
    }
}
