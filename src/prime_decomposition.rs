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
	println!("{:?}", factor(5) == vec!(5));
	println!("{:?}", factor(15) == vec!(3, 5));
	println!("{:?}", factor(16) == vec!(2, 2, 2, 2));
	println!("{:?}", factor(10287) == vec!(3, 3, 3, 3, 127));
}
