// Implements http://rosettacode.org/wiki/Input_loop
use std::io;

fn main() {
    for line in io::stdin().lock().lines() {
        print!("{}", line.unwrap());
    }
}
