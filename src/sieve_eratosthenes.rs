// Implements http://rosettacode.org/wiki/Sieve_of_Eratosthenes

use std::iter;

fn int_sqrt(n: uint) -> uint {
    (n as f64).sqrt() as uint
}

fn simple_sieve(limit: uint) -> ~[uint] {
    if limit < 2 {
        return ~[];
    }

    let mut primes = Vec::from_fn(limit + 1, |_| true);

    for prime in iter::range_inclusive(2, int_sqrt(limit) + 1) {
        if *primes.get(0) {
            for multiple in iter::range_step(prime * prime, limit + 1, prime) {
                *primes.get_mut(multiple) = false
            }
        }
    }
    
    iter::range_inclusive(2, limit).filter(|&n| *primes.get(n)).collect()
}

fn main() {
    println!("{:?}", simple_sieve(100))
}
