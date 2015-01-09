// Implements http://rosettacode.org/wiki/Sieve_of_Eratosthenes

use std::iter::{repeat, range_inclusive, range_step};
use std::num::Float;

fn int_sqrt(n: uint) -> uint {
    (n as f64).sqrt() as uint
}

// Return the prime numbers up to limit
fn simple_sieve(limit: uint) -> Vec<uint> {
    if limit < 2 {
        return vec!();
    }

    let mut primes: Vec<bool> = repeat(true).take(limit+1).collect();

    for prime in range_inclusive(2, int_sqrt(limit) + 1) {
        if primes[prime] {
            for multiple in range_step(prime * prime, limit + 1, prime) {
                primes[multiple] = false
            }
        }
    }

    range_inclusive(2, limit).filter(|&n| primes[n]).collect()
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
