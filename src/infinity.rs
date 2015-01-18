// Implements http://rosettacode.org/wiki/Infinity
#![allow(unstable)]
use std::num::Float;

fn main() {
    let inf : f32 = Float::infinity();
    println!("{}", inf);
}
