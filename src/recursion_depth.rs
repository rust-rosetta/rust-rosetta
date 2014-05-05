// Implements http://rosettacode.org/wiki/Find_limit_of_recursion

#[cfg(not(test))]
fn recursion(n: int) {
	println!("deep: {:d}", n);
	recursion(n + 1);
}

#[cfg(not(test))]
fn main() {
	recursion(0);
}
