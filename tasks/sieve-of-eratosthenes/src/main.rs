#![feature(iterator_step_by)]

use std::iter::repeat;

fn int_sqrt(n: usize) -> usize {
    (n as f64).sqrt() as usize
}

/// Return the prime numbers up to limit
#[cfg_attr(feature="clippy", allow(needless_range_loop))]
fn simple_sieve(limit: usize) -> Vec<usize> {
    if limit < 2 {
        return vec![];
    }

    let mut primes: Vec<bool> = repeat(true).take(limit + 1).collect();

    for prime in 2..(int_sqrt(limit) + 1) {
        if primes[prime] {
            for multiple in (prime * prime..limit + 1).step_by(prime) {
                primes[multiple] = false
            }
        }
    }

    (2..limit + 1).filter(|&n| primes[n]).collect()
}

fn main() {
    println!("{:?}", simple_sieve(100))
}

#[test]
fn test_basic() {
    let primes = simple_sieve(30);
    assert!(primes == [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
}
