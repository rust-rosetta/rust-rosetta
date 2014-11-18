// Implements http://rosettacode.org/wiki/Infinity

use std::num::Float;

fn main() {
    let inf : f32 = Float::infinity();
    println!("{}", inf);
}
