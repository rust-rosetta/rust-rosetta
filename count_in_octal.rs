use std::uint;

fn main() {
	for i in range(uint::MIN, uint::MAX) {
		println!("{:o}", i);
	}
}
