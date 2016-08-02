//! This task demonstrates three possible implementation strategies.

extern crate num;

use num::integer::gcd;

/// Iterative Euclid algorithm
fn iterative_euclid_gcd(mut m: i32, mut n: i32) -> i32 {
    while m != 0 {
        let old_m = m;
        m = n % m;
        n = old_m;
    }
    n.abs()
}

/// Recursive Euclid algorithm
fn recursive_euclid_gcd(m: i32, n: i32) -> i32 {
    if m == 0 {
        n.abs()
    } else {
        recursive_euclid_gcd(n % m, m)
    }
}

fn main() {
    println!("gcd(399, -3999) = {}", gcd(399, -3999));
    println!("gcd(0, 3999) = {}", iterative_euclid_gcd(0, 3999));
    println!("gcd(13 * 13, 13 * 29) = {}",
             recursive_euclid_gcd(13 * 13, 13 * 29));
}

#[cfg(test)]
mod tests {
    use super::{iterative_euclid_gcd, recursive_euclid_gcd};

    #[test]
    fn iterative() {
        assert_eq!(3, iterative_euclid_gcd(399, -3999));
        assert_eq!(3999, iterative_euclid_gcd(0, 3999));
        assert_eq!(13, iterative_euclid_gcd(13 * 13, 13 * 29));
    }

    #[test]
    fn recursive() {
        assert_eq!(3, recursive_euclid_gcd(399, -3999));
        assert_eq!(3999, recursive_euclid_gcd(0, 3999));
        assert_eq!(13, recursive_euclid_gcd(13 * 13, 13 * 29));
    }
}
