// Implements http://rosettacode.org/wiki/Loops/For

// not_tested

use std::iter;

fn main() {
    for i in iter::range_inclusive(1u, 5) {
        for _ in iter::range_inclusive(1u, i) {
            print!("*")
        }
        println!("")
    }
}
