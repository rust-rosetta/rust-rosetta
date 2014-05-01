// Implements http://rosettacode.org/wiki/Factorial

use std::iter::range_inclusive;

// Calculate the factorial using recursion
fn factorial_recursive (n: uint) -> uint {
	match n {
		0 => 1,
		_ => n * factorial_recursive(n-1)
	}
}

// Calculate the factorial using a fold
fn factorial_iterative(n: uint) -> uint {
	range_inclusive(1, n).fold(1, |p, t| p * t)
}

// Calculate the factorial using a for loop
fn factorial_loop(n: uint) -> uint {
    let mut fac = 1;
    for x in range_inclusive(1, n) {
        fac *= x;
    }
    fac
}

#[cfg(not(test))]
fn main () {
	for i in range(1u, 10) {
		println!("{}", factorial_recursive(i))
	}
	for i in range(1u, 10) {
		println!("{}", factorial_iterative(i))
	}
    for i in range(1u, 10) {
        println!("{}", factorial_loop(i));
    }
}

// Some tests and benchmarks
#[cfg(test)]
mod tests {
    extern crate test;
    use self::test::Bencher;
    use super::{factorial_recursive, factorial_iterative, factorial_loop};

    // Tests
    #[test]
    fn test_fac_recursive() {
        assert!(factorial_recursive(0) == 1);
        assert!(factorial_recursive(10) == 3628800);
    }

    #[test]
    fn test_fac_iterative() {
        assert!(factorial_iterative(0) == 1);
        assert!(factorial_iterative(10) == 3628800);
    }

    #[test]
    fn test_fac_loop() {
        assert!(factorial_loop(0) == 1);
        assert!(factorial_loop(10) == 3628800);
    }

    // Benchmarks
    #[bench]
    fn bench_fac_recursive(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(factorial_recursive(10));
        });
    }
    
    #[bench]
    fn bench_fac_iterative(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(factorial_iterative(10));
        });
    }
    
    #[bench]
    fn bench_fac_loop(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(factorial_loop(10));
        });
    }
}
