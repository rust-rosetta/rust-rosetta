// http://rosettacode.org/wiki/Evaluate_binomial_coefficients

fn binomial(n: uint, mut k: uint) -> u64 {
    // Since binomial(n, k) = binomial(n, n - k), we might as well use
    // the smaller k to optimize
    if n - k < k {
        k = n - k;
    }

    // Compute the coefficient
    let mut res = 1u64;
    for i in range(1, k as u64 + 1) {
        res *= (n - k) as u64 + i;
        res /= i;
    }

    res
}

#[cfg(not(test))]
fn main() {
    println!("{}", binomial(5, 3));
}

#[test]
fn test_binomial() {
    assert_eq!(binomial(20, 0), binomial(20, 20));
    assert_eq!(binomial(20, 15), binomial(19, 14) + binomial(19, 15));
    assert_eq!(binomial(5, 3), 10);
    assert_eq!(binomial(31, 17), 265182525);
}

