// http://rosettacode.org/wiki/Pernicious_numbers
use std::iter::{count, Filter, Counter};
use aks_test_for_primes::is_prime;
mod aks_test_for_primes;

#[cfg(not(test))]
fn main() {
    for i in pernicious().take(25) {
        print!("{} ", i);
    }
    println!("");
    for i in range(888_888_877u64, 888_888_888).filter(|&i| is_pernicious(i)) {
        print!("{} ", i);
    }
}

fn pernicious<'a>() -> Filter<'a, u64, Counter<u64>> {
    count(0u64, 1).filter(|&i| is_pernicious(i))
}

fn is_pernicious(n: u64) -> bool {
    is_prime(n.count_ones())
}

#[cfg(test)]
mod test {
    use super::{pernicious, is_pernicious};

    #[test]
    fn pernicious_iter() {
        let exp = &[3u64, 5, 6, 7, 9, 10, 11, 12, 13, 14, 17, 18, 19, 20, 21, 22,
                    24, 25, 26, 28, 31, 33, 34, 35, 36];
        for (act, &exp) in pernicious().take(30).zip(exp.iter()) {
            assert_eq!(act, exp);
        }
    }

    #[test]
    fn is_pernicious_range() {
        let exp = &[888888877u64, 888888878, 888888880, 888888883, 888888885, 888888886];
        for (act, &exp) in range(888_888_877u64, 888_888_888).filter(|&i| is_pernicious(i))
            .zip(exp.iter()) {
             assert_eq!(act, exp);
        }
    }
}