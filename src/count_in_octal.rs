// Implements http://rosettacode.org/wiki/Count_in_octal

use std::uint;

fn main() {
	for i in range(uint::MIN, uint::MAX) {
		println!("{:o}", i);
	}
}
