extern crate num;

use num::{BigUint, One};
use num::bigint::ToBigUint;
use num::integer::Integer;

fn mod_exp<T: Integer + Clone>(mut a: T, mut b: T, m: T) -> T {
    let one: T = One::one();
    let two = one.clone() + one.clone();

    let mut res = one;
    while !b.is_zero() {
        if b.is_odd() {
            res = (res * a.clone()) % m.clone();
        }
        a = (a.clone() * a.clone()) % m.clone();
        b = b.div_floor(&two);
    }

    res
}

fn main() {
    use num::pow;

    let a_str = "2988348162058574136915891421498819466320163312926952423791023078876139";
    let b_str = "2351399303373464486466122544523690094744975233415544072992656881240319";
    let a: BigUint = BigUint::parse_bytes(a_str.as_bytes(), 10).unwrap();
    let b: BigUint = BigUint::parse_bytes(b_str.as_bytes(), 10).unwrap();
    let m: BigUint = pow(10.to_biguint().unwrap(), 40);
    println!("{}", mod_exp(a, b, m));
}

#[test]
fn test_mod_exp() {
    let tests = [(0, 10, 10, 0),
                 (1, 10, 10, 1),
                 (2, 1000, 2, 0),
                 (2, 10, 2147483647, 1024),
                 (1337, 100, 2147483647, 1398068914),
                 (18, 112994442, 1000000001, 59108659)];

    for &(a, b, m, expected) in &tests {
        let a = a.to_biguint().unwrap();
        let b = b.to_biguint().unwrap();
        let m = m.to_biguint().unwrap();
        let ans: BigUint = mod_exp(a, b, m);

        assert_eq!(ans, expected.to_biguint().unwrap());
    }
}
