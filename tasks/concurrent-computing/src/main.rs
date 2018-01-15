extern crate rand;

use std::thread;
use std::time::Duration;

use rand::random;

fn main() {
    let strings = vec!["Enjoy", "Rosetta", "Code"];

    let mut children = vec![];

    for s in strings {
        children.push(thread::spawn(move || {
            // We use a random u8 (so an integer from 0 to 255)
            thread::sleep(Duration::from_millis(u64::from(random::<u8>())));
            println!("{}", s);
        }));
    }

    for child in children {
        child.join().unwrap();
    }
}
