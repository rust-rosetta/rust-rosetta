// Implements http://rosettacode.org/wiki/Assertions
#![allow(dead_code)] // not_tested

fn main() {
	let my_number = 42i;
	assert!(my_number == 42);
	assert_eq!(my_number, 42);
}
