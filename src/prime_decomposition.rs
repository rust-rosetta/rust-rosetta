// Implements http://rosettacode.org/wiki/Prime_decomposition

fn factor(mut nb: i32) -> Vec<i32> {
	let mut result = vec!();

	// First we take out all even factors.
	while nb % 2 == 0 {
		result.push(2);
		nb /= 2;
	}

	// Then (if any left) we take out the odd ones.
	let mut cand = 3;
	let mut max_bound = (nb as f32).sqrt() as i32 + 1;

	while cand <= max_bound {
		while nb % cand == 0 {
			result.push(cand);
			nb /= cand;
		}
		max_bound = (nb as f32).sqrt() as i32 + 1;
		cand += 2;
	}

	if nb > 1 {
		result.push(nb);
	}

	result
}

#[cfg(not(test))]
fn main() {
	println!("Factors of 5: {}", factor(5));
	println!("Factors of 15: {}", factor(15));
	println!("Factors of 16: {}", factor(16));
	println!("Factors of 10287: {}", factor(10287));
}

#[test]
fn test_basic() {
    assert!(factor(5) == vec!(5));
    assert!(factor(15) == vec!(3, 5));
    assert!(factor(16) == vec!(2, 2, 2, 2));
    assert!(factor(10287) == vec!(3, 3, 3, 3, 127));
}
