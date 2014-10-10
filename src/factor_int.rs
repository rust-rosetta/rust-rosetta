// Implements http://rosettacode.org/wiki/Factors_of_an_integer
use std::num;

#[cfg(not(test))]
fn main() {
    let target = 78i;
    println!("Factors of integer {}:", target);
    let factors = factor_int(target);
    
    for f in factors.iter() {
        println!("{}", f);
    }
}

// Compute the factors of an integer
// This method uses a simple check on each value between 1 and sqrt(x) to find
// pairs of factors
fn factor_int(x: int) -> Box<Vec<int>> {
    let mut factors: Vec<int> = Vec::new();
    
    // Compute the largest value to check i.e. sqrt(x)
    // Since we expect sane values rather than user input, we just fail if the
    // type conversions don't work
    let x_f: f64 = num::from_int(x).expect("Math fail!");
    let bound_f = x_f.sqrt();
    let bound: int = num::from_f64(bound_f.floor()).expect("Math fail!");
    
    for i in range(1i, bound) {
        if x % i == 0 {
            factors.push(i);
            factors.push(x/i);
        }
    }
    
    box factors
}

#[test]
fn test() {
    let result = factor_int(78i);
    assert_eq!(*result, vec![1i, 78, 2, 39, 3, 26, 6, 13]);
}
