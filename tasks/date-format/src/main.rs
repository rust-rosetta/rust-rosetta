extern crate chrono;

use chrono::*;

fn main() {
    let now: DateTime<UTC> = UTC::now();
    println!("{}", now.format("%Y-%m-%d").to_string());
    println!("{}", now.format("%A, %B %d, %Y").to_string());
}
