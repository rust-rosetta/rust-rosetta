fn main() {
    let target = 78i32;
    println!("Factors of integer {}:", target);
    let factors = factor_int(target);

    for f in factors {
        println!("{}", f);
    }
}

/// Compute the factors of an integer
/// This method uses a simple check on each value between 1 and sqrt(x) to find
/// pairs of factors
fn factor_int(x: i32) -> Vec<i32> {
    let mut factors: Vec<i32> = Vec::new();

    let bound: i32 = (x as f64).sqrt().floor() as i32;

    for i in 1i32..bound {
        if x % i == 0 {
            factors.push(i);
            factors.push(x / i);
        }
    }

    factors
}

#[test]
fn test() {
    let result = factor_int(78i32);
    assert_eq!(result, vec![1i32, 78, 2, 39, 3, 26, 6, 13]);
}
