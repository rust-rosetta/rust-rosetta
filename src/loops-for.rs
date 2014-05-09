// Implements http://rosettacode.org/wiki/Loops/For

#![cfg(not_tested)]

use std::iter;

fn main() {
    for i in iter::range_inclusive(1, 5) {
        for _ in iter::range_inclusive(1, i) {
            print!("*")
        }
        println!("")
    }
}
