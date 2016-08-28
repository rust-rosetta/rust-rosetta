//! Sleeps for the number of seconds specified on the command line.

use std::env;
use std::time::Duration;
use std::thread;

fn main() {
    let amount: u64 = env::args().nth(1).unwrap().parse().unwrap();
    println!("Sleeping...");
    thread::sleep(Duration::from_secs(amount));
    println!("Awake!");
}
