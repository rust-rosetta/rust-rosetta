#![feature(test)]
extern crate test;

/// Calculate the factorial using recursion
fn factorial_recursive(n: usize) -> usize {
    match n {
        0 => 1,
        _ => n * factorial_recursive(n - 1),
    }
}

/// Calculate the factorial using a fold
fn factorial_iterative(n: usize) -> usize {
    (1..n + 1).fold(1, |p, t| p * t)
}

/// Calculate the factorial using a for loop
fn factorial_loop(n: usize) -> usize {
    let mut fac = 1;
    for x in 1..n + 1 {
        fac *= x;
    }
    fac
}

fn main() {
    let fs = vec![("Recursive", factorial_recursive as fn(usize) -> usize),
                  ("Iterative", factorial_iterative as fn(usize) -> usize),
                  ("Looooooop", factorial_loop as fn(usize) -> usize)];
    for (name, f) in fs {
        println!("---------\n{}", name);
        for i in 1..10 {
            println!("{}", f(i))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{factorial_recursive, factorial_iterative, factorial_loop};
    use super::test::{self, Bencher};

    /// Tests
    fn t(f: fn(usize) -> usize) {
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
        t(factorial_recursive as fn(usize) -> usize)
    }

    #[test]
    fn test_fac_iterative() {
        t(factorial_iterative as fn(usize) -> usize)
    }

    #[test]
    fn test_fac_loop() {
        t(factorial_loop as fn(usize) -> usize)
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
