extern crate num;

use num::bigint::BigUint;
use num::traits::{Zero, One};
use num::integer::Integer;

fn mod_exp(b: &BigUint, e: &BigUint, n: &BigUint) -> Result<BigUint, &'static str> {
    if n.is_zero() {
        return Err("modulus is zero");
    }
    if b >= n {
        // base is too large and should be split into blocks
        return Err("base is >= modulus");
    }
    if b.gcd(n) != BigUint::one() {
        return Err("base and modulus are not relatively prime");
    }

    let mut bb = b.clone();
    let mut ee = e.clone();
    let mut result = BigUint::one();
    while !ee.is_zero() {
        if ee.is_odd() {
            result = (result * &bb) % n;
        }
        ee = ee >> 1;
        bb = (&bb * &bb) % n;
    }
    Ok(result)
}

fn main() {
    let msg = "Rosetta Code";

    let n = "9516311845790656153499716760847001433441357".parse().unwrap();
    let e = "65537".parse().unwrap();
    let d = "5617843187844953170308463622230283376298685".parse().unwrap();

    let msg_int = BigUint::from_bytes_be(msg.as_bytes());
    let enc = mod_exp(&msg_int, &e, &n).unwrap();
    let dec = mod_exp(&enc, &d, &n).unwrap();
    let msg_dec = String::from_utf8(dec.to_bytes_be()).unwrap();

    println!("msg as txt: {}", msg);
    println!("msg as num: {}", msg_int);
    println!("enc as num: {}", enc);
    println!("dec as num: {}", dec);
    println!("dec as txt: {}", msg_dec);
}

#[cfg(test)]
mod tests {
    use super::mod_exp;
    use num::bigint::BigUint;
    use num::integer::Integer;
    use num::traits::{Zero, FromPrimitive};

    const N: &'static str = "9516311845790656153499716760847001433441357";
    const E: &'static str = "65537";
    const D: &'static str = "5617843187844953170308463622230283376298685";

    fn rsa_numbers() -> (BigUint, BigUint, BigUint) {
        let n = N.parse().unwrap();
        let e = E.parse().unwrap();
        let d = D.parse().unwrap();
        (n, e, d)
    }

    #[test]
    fn test_enc_dec() {
        let (n, e, d) = rsa_numbers();
        let msg = "Rosetta Code";
        let msg_int = BigUint::from_bytes_be(msg.as_bytes());
        let enc = mod_exp(&msg_int, &e, &n).unwrap();
        let dec = mod_exp(&enc, &d, &n).unwrap();
        let msg_dec = String::from_utf8(dec.to_bytes_be()).unwrap();
        assert_eq!(msg, msg_dec);
    }

    #[test]
    fn test_enc_too_large_base() {
        let (n, e, _) = rsa_numbers();
        let msg = "I am too large for this modulus!";
        let msg_int = BigUint::from_bytes_be(msg.as_bytes());
        assert!(msg_int > n);
        let result = mod_exp(&msg_int, &e, &n);
        assert_eq!(Err("base is >= modulus"), result);
    }

    #[test]
    fn test_enc_zero_modulus() {
        let (_, e, _) = rsa_numbers();
        let msg_int = BigUint::from_bytes_be(b"msg");
        let result = mod_exp(&msg_int, &e, &BigUint::zero());
        assert_eq!(Err("modulus is zero"), result);
    }

    #[test]
    fn test_base_modulus_not_relatively_prime() {
        let (_, e, _) = rsa_numbers();
        let b = BigUint::from_u8(12).unwrap();
        let n = BigUint::from_u8(18).unwrap();
        assert_eq!(&BigUint::from_u8(6).unwrap(), &b.gcd(&n));
        let result = mod_exp(&b, &e, &n);
        assert_eq!(Err("base and modulus are not relatively prime"), result);
    }
}
