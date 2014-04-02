// Implements http://rosettacode.org/wiki/Loops/For

use std::iter;

fn main() {
    for i in iter::range_inclusive(1, 5) {
        for _j in iter::range_inclusive(1, i) {
            print!("*")
        }
        println!("")
    }
}
