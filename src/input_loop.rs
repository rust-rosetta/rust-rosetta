// Implements http://rosettacode.org/wiki/Input_loop
#![feature(old_io)]

use std::old_io;

fn main() {
    let mut stdin = old_io::stdin();
    for line in stdin.lock().lines() {
        print!("{}", line.unwrap());
    }
}
