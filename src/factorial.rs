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
    let fs = vec![("Recursive", factorial_recursive as fn(uint) -> uint),
                  ("Iterative", factorial_iterative as fn(uint) -> uint),
                  ("Looooooop", factorial_loop as fn(uint) -> uint)];
    for (name, f) in fs.into_iter() {
        println!("---------\n{}", name);
        for i in range(1u, 10) {
            println!("{}", f(i))
        }
    }
}

// Some tests and benchmarks
#[cfg(test)]
mod tests {
    extern crate test;
    use self::test::Bencher;
    use super::{factorial_recursive, factorial_iterative, factorial_loop};

    // Tests
    fn t<F>(f: F) where F: Fn(uint) -> uint {
        assert_eq!(f(0), 1);
        assert_eq!(f(1), 1);
        assert_eq!(f(2), 2);
        assert_eq!(f(3), 6);
        assert_eq!(f(4), 24);
        assert_eq!(f(5), 120);
        assert_eq!(f(6), 720);
        assert_eq!(f(7), 5040);
        assert_eq!(f(8), 40320);
        assert_eq!(f(9), 362880);
        assert_eq!(f(10), 3628800);
    }

    #[test]
    fn test_fac_recursive() {
        t(factorial_recursive)
    }

    #[test]
    fn test_fac_iterative() {
        t(factorial_iterative)
    }

    #[test]
    fn test_fac_loop() {
        t(factorial_loop)
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
