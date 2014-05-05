// Implements http://rosettacode.org/wiki/Loops/For

#[cfg(not(test))]
use std::iter;

#[cfg(not(test))]
fn main() {
    for i in iter::range_inclusive(1, 5) {
        for _ in iter::range_inclusive(1, i) {
            print!("*")
        }
        println!("")
    }
}
