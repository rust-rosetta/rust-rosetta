// Implements http://rosettacode.org/wiki/Proper_divisors
#![allow(unused_features)]

fn proper_divisors(n:i32) -> Vec<i32>{
	(1..(n / 2)+1).filter(|&i| n % i == 0).collect()
}

#[cfg(not(test))]
fn main() {
	for n in 1..11{
		println!("{:?}", proper_divisors(n));
	}

	let max_divisors = (1..20001)
		.map(|i| proper_divisors(i).len())
		.max().unwrap();

	println!("{}", max_divisors);
}

#[test]
fn test_divisors() {
    assert!(proper_divisors(6) == vec!(1,2,3));
    assert!(proper_divisors(100) == vec!(1,2,4,5,10,20,25,50));
}