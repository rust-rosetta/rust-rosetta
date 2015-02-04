// Implements http://rosettacode.org/wiki/Input_loop
#![feature(io)]
#![feature(core)]

use std::old_io;

fn main() {
    for line in old_io::stdin().lock().lines() {
        print!("{}", line.unwrap());
    }
}
