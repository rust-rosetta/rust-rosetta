// Implements http://rosettacode.org/wiki/Sieve_of_Eratosthenes
#![feature(core)]

use std::iter::{repeat, range_step};
use std::num::Float;

fn int_sqrt(n: usize) -> usize {
    (n as f64).sqrt() as usize
}

// Return the prime numbers up to limit
fn simple_sieve(limit: usize) -> Vec<usize> {
    if limit < 2 {
        return vec!();
    }

    let mut primes: Vec<bool> = repeat(true).take(limit+1).collect();

    for prime in (2..int_sqrt(limit) + 1 + 1) {
        if primes[prime] {
            for multiple in range_step(prime * prime, limit + 1, prime) {
                primes[multiple] = false
            }
        }
    }

    (2..limit + 1).filter(|&n| primes[n]).collect()
}

#[cfg(not(test))]
fn main() {
    println!("{:?}", simple_sieve(100))
}

#[test]
fn test_basic() {
    let primes = simple_sieve(30);
    assert!(primes == [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
}
