// Implements http://rosettacode.org/wiki/Factors_of_an_integer

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
fn factor_int(x: int) -> Vec<int> {
    let mut factors: Vec<int> = Vec::new();
    
    let bound: int = (x as f64).sqrt().floor() as int;
    
    for i in range(1i, bound) {
        if x % i == 0 {
            factors.push(i);
            factors.push(x/i);
        }
    }
    
    factors
}

#[test]
fn test() {
    let result = factor_int(78i);
    assert_eq!(result, vec![1i, 78, 2, 39, 3, 26, 6, 13]);
}
