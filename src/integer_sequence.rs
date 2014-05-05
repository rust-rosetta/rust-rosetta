// Implements http://rosettacode.org/wiki/Integer_sequence

extern crate num;

#[cfg(not(test))]
use num::bigint::BigUint;
#[cfg(not(test))]
use std::num::One;

#[cfg(not(test))]
fn main() {
	let one: BigUint = One::one();
	let mut i: BigUint = One::one();

	loop {
		println!("{:s}", i.to_str());
		i = i + one;
	}
}
