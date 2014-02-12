fn recursion(n: int) {
	println(fmt!("%?", n));
	recursion(n + 1);
}

fn main() {
	recursion(0);
}
