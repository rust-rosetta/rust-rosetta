// Implements http://rosettacode.org/wiki/Find_limit_of_recursion
#![cfg(not_tested)]

fn recursion(n: int) {
	println!("deep: {:d}", n);
	recursion(n + 1);
}

fn main() {
	recursion(0);
}
