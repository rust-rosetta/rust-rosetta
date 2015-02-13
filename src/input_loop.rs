// Implements http://rosettacode.org/wiki/Input_loop
#![feature(io)]
#![feature(core)]

use std::old_io;

fn main() {
	let mut stdin = old_io::stdin();
	let mut lock = stdin.lock();

    for line in lock.lines() {
        print!("{}", line.unwrap());
    }
}
