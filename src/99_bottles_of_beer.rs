// Implements http://rosettacode.org/wiki/99_Bottles_of_Beer
#![cfg(not_tested)]
use std::iter::range_step_inclusive;

fn sing_bottles_line(num_bottles: int, on_the_wall: bool) {
	// the print! macro uses a built in internationalization formatting language
	// check out the docs for std::fmt
	print!("{0, plural, =0{No bottles} =1{One bottle} other{# bottles}} of beer", num_bottles as uint);

	if on_the_wall {
		print!(" on the wall!");
	}

	print!("\n");
}

fn main() {
	for num_bottles in range_step_inclusive(99, 1, -1) {
		sing_bottles_line(num_bottles, true);
		sing_bottles_line(num_bottles, false);
		println!("Take one down, pass it around...");
		sing_bottles_line(num_bottles - 1, true);
		println!("-----------------------------------");
	}
}
