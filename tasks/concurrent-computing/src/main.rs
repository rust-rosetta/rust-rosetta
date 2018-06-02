extern crate rand;

use std::thread;
use std::time::Duration;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut thread_spawn = |string| {
        // We use a random u8 (so an integer from 0 to 255)
        let duration = u64::from(rng.gen::<u8>());
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(duration));
            println!("{}", string);
        })
    };

    let children = vec![
        thread_spawn("Enjoy"),
        thread_spawn("Rosetta"),
        thread_spawn("Code"),
    ];

    for child in children {
        child.join().unwrap();
    }
}
