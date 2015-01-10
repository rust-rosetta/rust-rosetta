// Implements http://rosettacode.org/wiki/Loops/For


use std::iter;

fn main() {
    for i in iter::range_inclusive(1us, 5) {
        for _ in iter::range_inclusive(1us, i) {
            print!("*")
        }
        println!("")
    }
}
