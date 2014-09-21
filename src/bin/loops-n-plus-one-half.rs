// Implements http://rosettacode.org/wiki/Loops/N_plus_one_half
#![allow(dead_code)] // not_tested

use std::iter;

fn main() {
    for i in iter::range_inclusive(1u,10) {
        print!("{}", i);
        if i == 10 {
            break;
        }
        print!(", ");
    }
}
