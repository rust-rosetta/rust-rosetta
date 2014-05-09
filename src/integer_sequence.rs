// Implements http://rosettacode.org/wiki/Integer_sequence
#![cfg(not_tested)]

extern crate num;

use num::bigint::BigUint;
use std::num::One;

fn main() {
	let one: BigUint = One::one();
	let mut i: BigUint = One::one();

	loop {
		println!("{:s}", i.to_str());
		i = i + one;
	}
}
