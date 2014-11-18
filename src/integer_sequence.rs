// Implements http://rosettacode.org/wiki/Integer_sequence

extern crate num;

use num::bigint::BigUint;
use num::traits::One;

fn main() {
    let one: BigUint = One::one();
    let mut i: BigUint = One::one();

    loop {
        println!("{}", i);
        i = i + one;
    }
}
