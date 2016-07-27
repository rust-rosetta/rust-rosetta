extern crate aks_test_for_primes;

use std::iter::Filter;
use std::ops::RangeFrom;

use aks_test_for_primes::is_prime;

fn main() {
    for i in pernicious().take(25) {
        print!("{} ", i);
    }
    println!("");
    for i in (888_888_877u64..888_888_888).filter(is_pernicious) {
        print!("{} ", i);
    }
}

fn pernicious() -> Filter<RangeFrom<u64>, fn(&u64) -> bool> {
    (0u64..).filter(is_pernicious as fn(&u64) -> bool)
}

fn is_pernicious(n: &u64) -> bool {
    is_prime(n.count_ones())
}

#[cfg(test)]
mod tests {
    use super::{is_pernicious, pernicious};

    #[test]
    fn pernicious_iter() {
        let exp = &[3u64, 5, 6, 7, 9, 10, 11, 12, 13, 14, 17, 18, 19, 20, 21, 22, 24, 25, 26, 28,
                    31, 33, 34, 35, 36];
        for (act, &exp) in pernicious().take(30).zip(exp.iter()) {
            assert_eq!(act, exp);
        }
    }

    #[test]
    fn is_pernicious_range() {
        let exp = &[888888877u64, 888888878, 888888880, 888888883, 888888885, 888888886];
        for (act, &exp) in (888_888_877u64..888_888_888)
            .filter(is_pernicious)
            .zip(exp.iter()) {
            assert_eq!(act, exp);
        }
    }
}
