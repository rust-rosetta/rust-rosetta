// Implements http://rosettacode.org/wiki/Almost_prime

use std::iter::{count, range_inclusive};

fn is_kprime(mut n: uint, k: uint) -> bool {
    let mut p = 2;
    let mut f = 0;

    while f < k && p * p <= n {
        while 0 == n % p {
            n /= p;
            f += 1;
        }
        p += 1;
    }

    f + (n > 1) as uint == k
}

fn get_kprimes(k: uint, amount: uint) -> Vec<uint> {   
    count(2u, 1).filter(|&x| is_kprime(x, k))
                .take(amount)
                .collect()
}

fn main() {
    for k in range_inclusive(1u, 5) {
        println!("k = {}: {}", k, get_kprimes(k, 10));
    }
}

#[test]
fn test_almost_primes() {
    // k = 1
    assert!(get_kprimes(1, 10).as_slice() == [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    
    // k = 2
    assert!(get_kprimes(2, 10).as_slice() == [4, 6, 9, 10, 14, 15, 21, 22, 25, 26]);
    
    // k = 3
    assert!(get_kprimes(3, 10).as_slice() == [8, 12, 18, 20, 27, 28, 30, 42, 44, 45]);
    
    // k = 4
    assert!(get_kprimes(4, 10).as_slice() == [16, 24, 36, 40, 54, 56, 60, 81, 84, 88]);
    
    // k = 5
    assert!(get_kprimes(5, 10).as_slice() == [32, 48, 72, 80, 108, 112, 120, 162, 168, 176]);
}