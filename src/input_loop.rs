// Implements http://rosettacode.org/wiki/Input_loop
#![feature(io)]

use std::old_io;

fn main() {
    for line in old_io::stdin().lock().lines() {
        print!("{}", line.unwrap());
    }
}
