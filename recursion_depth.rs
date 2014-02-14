fn recursion(n: int) {
	println!("deep: {:d}", n);
	recursion(n + 1);
}

fn main() {
	recursion(0);
}
