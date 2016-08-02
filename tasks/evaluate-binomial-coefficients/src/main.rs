extern crate num;

use num::One;
use num::bigint::{BigUint, ToBigUint};

fn binomial(n: usize, mut k: usize) -> BigUint {
    // Since binomial(n, k) = binomial(n, n - k), we might as well use
    // the smaller k to optimize
    if n - k < k {
        k = n - k;
    }

    // Compute the coefficient
    let mut res: BigUint = One::one();
    for i in 1..(k + 1) {
        let m: BigUint = (n - k + i).to_biguint().unwrap();
        res = res * m;
        let d: BigUint = (i).to_biguint().unwrap();
        res = res / d;
    }

    res
}

fn main() {
    println!("{}", binomial(5, 3));
}

#[test]
fn test_binomial() {
    use num::traits::Num;

    assert_eq!(binomial(20, 0), binomial(20, 20));
    assert_eq!(binomial(20, 15), binomial(19, 14) + binomial(19, 15));
    assert_eq!(binomial(5, 3), 10.to_biguint().unwrap());
    assert_eq!(binomial(31, 17), 265182525.to_biguint().unwrap());
    assert_eq!(binomial(300, 30),
               BigUint::from_str_radix("173193226149263513034110205899732811401360", 10).unwrap());
}
