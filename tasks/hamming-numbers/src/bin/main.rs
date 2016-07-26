extern crate num;

extern crate hamming_numbers;

use num::BigUint;
use num::bigint::ToBigUint;

use hamming_numbers::Hamming;

fn main() {
    // capacity of the queue currently needs to be a power of 2 because of a bug with VecDeque
    let hamming: Hamming<BigUint> = Hamming::new(128);

    for (idx, h) in hamming.enumerate().take(1_000_000) {
        match idx + 1 {
            1...20 => print!("{} ", h.to_biguint().unwrap()),
            i @ 1691 | i @ 1000000 => println!("\n{}th number: {}", i, h.to_biguint().unwrap()),
            _ => continue,
        }
    }
}
