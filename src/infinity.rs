// Implements http://rosettacode.org/wiki/Infinity
#![feature(std_misc)]
use std::num::Float;

fn main() {
    let inf : f32 = Float::infinity();
    println!("{}", inf);
}
