//! Alternate version: uses a more efficient representation of Hamming numbers:
//! instead of storing them as `BigUint` directly, it stores the three exponents
//! i, j and k for 2^i * 3^j * 5 ^k and the logarithm of the number for comparisons

extern crate num;

extern crate hamming_numbers;

use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::ops::Mul;

use num::bigint::{BigUint, ToBigUint};
use num::pow;
use num::traits::One;

use hamming_numbers::{Hamming, HammingNumber};

fn main() {
    // capacity of the queue currently needs to be a power of 2 because of a bug with VecDeque
    let hamming: Hamming<HammingTriple> = Hamming::new(128);

    for (idx, h) in hamming.enumerate().take(1_000_000) {
        match idx + 1 {
            1...20 => print!("{} ", h.to_biguint().unwrap()),
            i @ 1691 | i @ 1_000_000 => println!("\n{}th number: {}", i, h.to_biguint().unwrap()),
            _ => continue,
        }
    }
}

// we store these to calculate the ln of a hamming number
pub const LN_2: f64 =
    0.693_147_180_559_945_309_417_232_121_458_176_568_075_500_134_360_255_254_120_680;
pub const LN_3: f64 =
    1.098_612_288_668_109_691_395_245_236_922_525_704_647_490_557_822_749_451_734_694;
pub const LN_5: f64 =
    1.609_437_912_434_100_374_600_759_333_226_187_639_525_601_354_268_517_721_912_647;

/// more space-efficient representation of a Hamming number.
/// A Hamming number is 2^i * 3^j * 5^k;
/// instead of storing it directly as a `BigUint`
/// we store the powers i, j and k and calculate the
/// result as a `BigUint` only when we need it.
/// we also store the logarithm for quicker comparisons, using this property
/// of logarithms: ln(2^i * 3^j * 5^k) = i*ln2 + j*ln3 + k*ln5
#[derive(Debug, Copy, Clone)]
pub struct HammingTriple {
    pow_2: usize,
    pow_3: usize,
    pow_5: usize,
    ln: f64,
}

impl Mul for HammingTriple {
    type Output = HammingTriple;
    fn mul(self, other: HammingTriple) -> HammingTriple {
        HammingTriple {
            pow_2: self.pow_2 + other.pow_2,
            pow_3: self.pow_3 + other.pow_3,
            pow_5: self.pow_5 + other.pow_5,
            ln: self.ln + other.ln,
        }
    }
}

impl One for HammingTriple {
    /// 1 as an HammingNumber is 2^0 * 3^0 * 5^0
    /// ln(1) = 0
    fn one() -> HammingTriple {
        HammingTriple::new(0, 0, 0)
    }
}

impl HammingNumber for HammingTriple {
    fn multipliers() -> (HammingTriple, HammingTriple, HammingTriple) {
        (
            HammingTriple {
                pow_2: 1,
                pow_3: 0,
                pow_5: 0,
                ln: LN_2,
            },
            HammingTriple {
                pow_2: 0,
                pow_3: 1,
                pow_5: 0,
                ln: LN_3,
            },
            HammingTriple {
                pow_2: 0,
                pow_3: 0,
                pow_5: 1,
                ln: LN_5,
            },
        )
    }
}

impl ToBigUint for HammingTriple {
    /// calculate the value as a `BigUint`
    fn to_biguint(&self) -> Option<BigUint> {
        Some(
            pow(2u8.to_biguint().unwrap(), self.pow_2)
                * pow(3u8.to_biguint().unwrap(), self.pow_3)
                * pow(5u8.to_biguint().unwrap(), self.pow_5),
        )
    }
}

impl HammingTriple {
    fn new(pow_2: usize, pow_3: usize, pow_5: usize) -> HammingTriple {
        HammingTriple {
            pow_2: pow_2,
            pow_3: pow_3,
            pow_5: pow_5,
            ln: (pow_2 as f64) * LN_2 + (pow_3 as f64) * LN_3 + (pow_5 as f64) * LN_5,
        }
    }
}

impl PartialEq for HammingTriple {
    fn eq(&self, other: &HammingTriple) -> bool {
        self.pow_2 == other.pow_2 && self.pow_3 == other.pow_3 && self.pow_5 == other.pow_5
    }
}

impl Eq for HammingTriple {}

impl PartialOrd for HammingTriple {
    fn partial_cmp(&self, other: &HammingTriple) -> Option<Ordering> {
        if self == other {
            Some(Equal)
        } else if ((self.pow_2 >= other.pow_2)
            && (self.pow_3 >= other.pow_3)
            && (self.pow_5 >= other.pow_5))
            || (self.ln > other.ln)
        {
            Some(Greater)
        } else if ((self.pow_2 <= other.pow_2)
            && (self.pow_3 <= other.pow_3)
            && (self.pow_5 <= other.pow_5))
            || (self.ln < other.ln)
        {
            Some(Less)
        } else {
            None
        }
    }
}

impl Ord for HammingTriple {
    fn cmp(&self, other: &HammingTriple) -> Ordering {
        // as a last resort we need to calculate the BigUint values and compare them.
        // This should be rare. The reason is that for very big values floating point precision
        // could make hamming_1.ln == hamming_2.ln even if the two numbers are actually different
        self.partial_cmp(other)
            .unwrap_or_else(|| self.to_biguint().unwrap().cmp(&other.to_biguint().unwrap()))
    }
}

#[test]
fn hamming_iter() {
    let mut hamming = Hamming::<HammingTriple>::new(20);
    assert!(hamming.nth(19).unwrap().to_biguint() == 36u8.to_biguint());
}

#[test]
fn hamming_iter_1million() {
    let mut hamming = Hamming::<HammingTriple>::new(128);
    let millionth_hamming_number = "51931278044838873608958984375000000000000000000000000000000000\
                                    0000000000000000000000";

    // one-million-th hamming number has index 999_999 because indexes are zero-based
    assert_eq!(
        hamming.nth(999_999).unwrap().to_biguint(),
        millionth_hamming_number.parse::<BigUint>().ok()
    );
}
