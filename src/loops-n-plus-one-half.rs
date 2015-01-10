// Implements http://rosettacode.org/wiki/Loops/N_plus_one_half

use std::iter;

fn main() {
    for i in iter::range_inclusive(1us,10) {
        print!("{:?}", i);
        if i == 10 {
            break;
        }
        print!(", ");
    }
}
