// Implements http://rosettacode.org/wiki/Sieve_of_Eratosthenes

use std::iter::{range_inclusive, range_step};

fn int_sqrt(n: uint) -> uint {
    (n as f64).sqrt() as uint
}

// Return the prime numbers up to limit
fn simple_sieve(limit: uint) -> Vec<uint> {
    if limit < 2 {
        return vec!();
    }

    let mut primes = Vec::from_elem(limit + 1, true);

    for prime in range_inclusive(2, int_sqrt(limit) + 1) {
        if *primes.get(prime) {
            for multiple in range_step(prime * prime, limit + 1, prime) {
                *primes.get_mut(multiple) = false
            }
        }
    }

    range_inclusive(2, limit).filter(|&n| *primes.get(n)).collect()
}

#[cfg(not(test))]
fn main() {
    println!("{}", simple_sieve(100))
}

#[test]
fn test_basic() {
    let primes = simple_sieve(30);
    assert!(primes.as_slice() == [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
}
