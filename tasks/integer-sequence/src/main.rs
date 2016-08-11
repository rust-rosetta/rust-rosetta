extern crate num;

use num::{BigUint, One};

fn main() {
    let one: BigUint = One::one();
    let mut i: BigUint = One::one();

    loop {
        println!("{}", i);
        i = &i + &one;
    }
}
