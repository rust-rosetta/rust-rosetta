// Implements http://rosettacode.org/wiki/Count_in_octal

#[cfg(not(test))]
use std::u8;
#[cfg(not(test))]
use std::iter::range_inclusive;

#[cfg(not(test))]
fn main() {
    // We count from 0 to 255 (377 in octal)
	for i in range_inclusive(0, u8::MAX) {
		println!("{:o}", i);
	}
}
