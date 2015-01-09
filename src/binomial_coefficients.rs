// http://rosettacode.org/wiki/Evaluate_binomial_coefficients

extern crate num;
extern crate core;
use num::{BigUint, One};
use std::num::FromPrimitive;

fn binomial(n: usize, mut k: usize) -> BigUint {
    // Since binomial(n, k) = binomial(n, n - k), we might as well use
    // the smaller k to optimize
    if n - k < k {
        k = n - k;
    }

    // Compute the coefficient
    let mut res: BigUint = One::one();
    for i in range(1us, k + 1) {
        let m: BigUint = FromPrimitive::from_uint(n - k + i).unwrap();
        res = res * m;
        let d: BigUint = FromPrimitive::from_uint(i).unwrap();
        res = res / d;
    }

    res
}

#[cfg(not(test))]
fn main() {
    println!("{:?}", binomial(5, 3));
}

#[test]
fn test_binomial() {
    use std::str::FromStr;

    assert_eq!(binomial(20, 0), binomial(20, 20));
    assert_eq!(binomial(20, 15), binomial(19, 14) + binomial(19, 15));
    assert_eq!(binomial(5, 3), FromPrimitive::from_uint(10).unwrap());
    assert_eq!(binomial(31, 17), FromPrimitive::from_uint(265182525).unwrap());
    assert_eq!(binomial(300, 30),
        FromStr::from_str("173193226149263513034110205899732811401360").unwrap());
}

