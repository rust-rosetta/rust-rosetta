extern crate num;

use num::{BigInt, Signed};

// Dumbest iterative approach
fn big_pow(base: &BigInt, exp: BigInt) -> BigInt {
    if exp.is_negative() {
        panic!("Negative exponent won't compute!")
    }
    let mut tmp = base.clone();
    for _ in num::range(BigInt::from(1), exp) {
        tmp = tmp * base;
    }
    tmp
}

// 5^4^3^2
fn main() {
    // Exponent is small enough to not use BigInt
    let exp = BigInt::from(num::pow(4, num::pow(3, 2)));

    let result = big_pow(&BigInt::from(5), exp).to_string();
    let num_length = result.len();

    println!("{}", result);
    println!("Number has {} digits.", num_length);
    assert!(result.starts_with("62060698786608744707"));
    assert!(result.ends_with("92256259918212890625"));
}

#[cfg(test)]
mod tests {
    use super::big_pow;
    use num::BigInt;

    #[test]
    #[should_panic]
    fn negative_exp_test() {
        let num = BigInt::from(100);
        let exp = BigInt::from(-100);
        big_pow(&num, exp);
    }

    #[test]
    fn big_powas() {
        assert_eq!(big_pow(&BigInt::from(100), BigInt::from(100)).to_string(),
            "10000000000000000000000000000000000000000000000000000000000000\
            000000000000000000000000000000000000000000000000000000000000000\
            000000000000000000000000000000000000000000000000000000000000000\
            0000000000000");

        assert_eq!(big_pow(&BigInt::from(2), BigInt::from(89)).to_string(),
            "618970019642690137449562112");

        assert_eq!(big_pow(&BigInt::from(2), BigInt::from(107)).to_string(),
            "162259276829213363391578010288128");

        assert_eq!(big_pow(&BigInt::from(2), BigInt::from(127)).to_string(),
            "170141183460469231731687303715884105728");

        assert_eq!(big_pow(&BigInt::from(2), BigInt::from(521)).to_string(),
            "6864797660130609714981900799081393217269435300143305409394\
            46345918554318339765605212255964066145455497729631139148085\
            8037121987999716643812574028291115057152");
    }
}
