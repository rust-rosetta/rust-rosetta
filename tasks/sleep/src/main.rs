//! Sleeps for the number of seconds specified on the command line.

use std::env;
use std::thread;
use std::time::Duration;

fn main() {
    let amount: u64 = env::args().nth(1).unwrap().parse().unwrap();
    println!("Sleeping...");
    thread::sleep(Duration::from_secs(amount));
    println!("Awake!");
}
