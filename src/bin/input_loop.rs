// Implements http://rosettacode.org/wiki/Input_loop
#![allow(dead_code)] // not_tested
use std::io;

fn main() {
	for line in io::stdin().lines() {
	    print!("{}", line.unwrap());
	}
}
