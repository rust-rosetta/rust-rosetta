use std::uint;

fn main() {
	for i in range(uint::min_value, uint::max_value) {
		println(fmt!("%o", i));
	}
}
