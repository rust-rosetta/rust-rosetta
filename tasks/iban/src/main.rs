extern crate num;

use num::bigint::{BigInt, ToBigInt};

fn main() {
    if is_valid("GB82 WEST 1234 5698 7654 32") {
        println!("IBAN correctly validated!");
    } else {
        println!("Invalid IBAN!");
    }
}

/// Returns true if the IBAN is valid
/// See [Validating the IBAN][iban].
///
/// [iban]: http://en.wikipedia.org/wiki/International_Bank_Account_Number#Validating_the_IBAN
fn is_valid(iban: &str) -> bool {
    // Discard whitespace
    let mut iban_chars: Vec<char> = iban.chars().filter(|c| !c.is_whitespace()).collect();

    if iban_chars.len() < 2 {
        return false;
    }

    // Check length of the IBAN
    match country_length(&iban[..2]) {
        Some(l) if l == iban_chars.len() => {}
        _ => return false,
    };

    // Rearrange (first four characters go to the back)
    for _ in 0..4 {
        let front = iban_chars.remove(0);
        iban_chars.push(front);
    }

    // Expand letters to digits
    let iban_int = parse_digits(&iban_chars);

    // Check if the remainder is one
    match iban_int {
        Some(x) => x % 97.to_bigint().unwrap() == 1.to_bigint().unwrap(),
        None => false,
    }
}

/// Returns a `BigInt` made from the digits and letters of the IBAN
fn parse_digits(chars: &[char]) -> Option<BigInt> {
    let mut vec: Vec<u8> = Vec::with_capacity(chars.len() + 10);

    // Copy the digits to the vector and expand the letters to digits
    for &c in chars {
        match c.to_digit(36) {
            Some(d) => vec.extend(d.to_string().bytes()),
            None => return None,
        }
    }
    let as_str = String::from_utf8(vec).unwrap(); // since it was built
        // from digits we know the vec is all made of valid utf8, so we
        // can just unwrap()
    as_str.parse::<BigInt>().ok()
}

fn country_length(country_code: &str) -> Option<usize> {
    let countries = [("AL", 28), ("AD", 24), ("AT", 20), ("AZ", 28), ("BE", 16), ("BH", 22),
                     ("BA", 20), ("BR", 29), ("BG", 22), ("CR", 21), ("HR", 21), ("CY", 28),
                     ("CZ", 24), ("DK", 18), ("DO", 28), ("EE", 20), ("FO", 18), ("FI", 18),
                     ("FR", 27), ("GE", 22), ("DE", 22), ("GI", 23), ("GR", 27), ("GL", 18),
                     ("GT", 28), ("HU", 28), ("IS", 26), ("IE", 22), ("IL", 23), ("IT", 27),
                     ("KZ", 20), ("KW", 30), ("LV", 21), ("LB", 28), ("LI", 21), ("LT", 20),
                     ("LU", 20), ("MK", 19), ("MT", 31), ("MR", 27), ("MU", 30), ("MC", 27),
                     ("MD", 24), ("ME", 22), ("NL", 18), ("NO", 15), ("PK", 24), ("PS", 29),
                     ("PL", 28), ("PT", 25), ("RO", 24), ("SM", 27), ("SA", 24), ("RS", 22),
                     ("SK", 24), ("SI", 19), ("ES", 24), ("SE", 24), ("CH", 21), ("TN", 24),
                     ("TR", 26), ("AE", 23), ("GB", 22), ("VG", 24)];

    countries.iter()
        .find(|&&(country, _)| country == country_code)
        .map(|&(_, length)| length)
}

#[test]
fn test_valid() {
    assert!(is_valid("GB82 WEST 1234 5698 7654 32"));
    assert!(is_valid("BE18 0016 5492 3565"));
}

#[test]
fn test_wrong() {
    assert!(!is_valid("ASDA FJAS DMAF BKDB AKGS DH"));
    assert!(!is_valid("XX82 WEST 1234 5698 7654 32"));
    assert!(!is_valid("BE18 0016 5492 3566"));
    assert!(!is_valid("BE18 0016 5492 3565 6"));
    assert!(!is_valid("BE18 0016 5492 356"));
    assert!(!is_valid(""));
}
