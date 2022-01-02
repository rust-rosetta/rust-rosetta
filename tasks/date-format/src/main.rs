extern crate chrono;

use chrono::*;

fn main() {
    let now = Utc::now();
    println!("{}", now.format("%Y-%m-%d"));
    println!("{}", now.format("%A, %B %d, %Y"));
}
