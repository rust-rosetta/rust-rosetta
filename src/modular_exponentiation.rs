// http://rosettacode.org/wiki/Modular_exponentiation
extern crate core;
extern crate num;
use num::traits::One;
use num::bigint::BigUint;
use num::integer::Integer;

fn mod_exp<T: Integer>(mut a: T, mut b: T, m: T) -> T {
    let one: T = One::one();
    let two = one + one;

    let mut res = one;
    while !b.is_zero() {
        if b.is_odd() {
            res = (res * a) % m;
        }
        a = (a * a) % m;
        b = b.div_floor(&two);
    }

    res
}

#[cfg(not(test))]
fn main() {
    use num::pow;
    use std::str::FromStr;

    let a_str = "2988348162058574136915891421498819466320163312926952423791023078876139";
    let b_str = "2351399303373464486466122544523690094744975233415544072992656881240319";
    let a: BigUint = FromStr::from_str(a_str).unwrap();
    let b: BigUint = FromStr::from_str(b_str).unwrap();
    let m: BigUint = pow(FromPrimitive::from_int(10).unwrap(), 40);
    println!("{}", mod_exp(a, b, m));
}

#[test]
fn test_mod_exp() {
    let tests = [
        (0, 10, 10, 0),
        (1, 10, 10, 1),
        (2, 1000, 2, 0),
        (2, 10, 2147483647, 1024),
        (1337, 100, 2147483647, 1398068914),
        (18, 112994442, 1000000001, 59108659),
    ];

    for &(a, b, m, expected) in tests.iter() {
        let a = FromPrimitive::from_int(a).unwrap();
        let b = FromPrimitive::from_int(b).unwrap();
        let m = FromPrimitive::from_int(m).unwrap();
        let ans: BigUint = mod_exp(a, b, m);

        assert_eq!(ans, FromPrimitive::from_int(expected).unwrap());
    }
}

