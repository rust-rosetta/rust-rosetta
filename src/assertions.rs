// Implements http://rosettacode.org/wiki/Assertions
// not_tested

fn main() {
	let my_number = 42i;
	assert!(my_number == 42i);
	assert_eq!(my_number, 42i);
}
