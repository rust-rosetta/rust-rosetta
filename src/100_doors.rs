// Implements http://rosettacode.org/wiki/100_doors
#![cfg(not_tested)]
fn main() {
	for i in std::iter::range_inclusive(1,100) {
		let x = (i as f64).powf(&0.5);
		let state = if x == x.round() {"open"} else {"closed"};
		println!("Door {} is {}", i, state);
	}
}
