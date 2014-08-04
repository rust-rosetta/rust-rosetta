extern crate num;

use hamming_numbers::{Hamming, HammingNumber};
use std::num;
use num::bigint::{BigUint, ToBigUint};

mod hamming_numbers;

#[cfg(not(test))]
fn main() {
    let hamming : Hamming<HammingTriple> = Hamming::new(100);

    for (idx, h) in hamming.enumerate().take(1_000_000) {
        match idx + 1 {
            1..20 => print!("{} ", h.to_biguint().unwrap()),
            i @ 1691 | i @ 1000000 => println!("\n{}th number: {}", i, h.to_biguint().unwrap()),
            _ =>  continue
        }
    }
}

// we store these to calculate the ln of a hamming number
pub static LN_2: f64 = 0.693147180559945309417232121458176568075500134360255254120680_f64;
pub static LN_3: f64 = 1.098612288668109691395245236922525704647490557822749451734694_f64;
pub static LN_5: f64 = 1.609437912434100374600759333226187639525601354268517721912647_f64;

// more space-efficient representation of a Hamming number.
// A Hamming number is 2^i * 3^j * 5^k;
// instead of storing it directly as a BigUint
// we store the powers i, j and k and calculate the
// result as a BigUint only when we need it.
// we also store the logarithm for quicker comparisons, using this property
// of logarithms: ln(2^i * 3^j * 5^k) = i*ln2 + j*ln3 + k*ln5
#[deriving(Show)]
pub struct HammingTriple {
    pow_2: uint,
    pow_3: uint,
    pow_5: uint,
    ln: f64
}

impl HammingNumber for HammingTriple {
    fn times_2(&self) -> HammingTriple {
        HammingTriple {
            pow_2: self.pow_2 + 1, pow_3: self.pow_3, pow_5: self.pow_5, ln: self.ln + LN_2 }
    }

    fn times_3(&self) -> HammingTriple {
        HammingTriple {
            pow_2: self.pow_2, pow_3: self.pow_3 + 1, pow_5: self.pow_5, ln: self.ln +  LN_3 }
    }

    fn times_5(&self) -> HammingTriple {
        HammingTriple {
            pow_2: self.pow_2, pow_3: self.pow_3, pow_5: self.pow_5 + 1, ln: self.ln +  LN_5 }
    }

    // 1 as an HammingNumber is 2^0 * 3^0 * 5^0
    // ln(1) = 0
    fn one() -> HammingTriple {
        HammingTriple::new(0, 0, 0)
    }
}

impl ToBigUint for HammingTriple {
   // calculate the value as a BigUint
    fn to_biguint(&self) -> Option<BigUint> {
        Some(num::pow(2u.to_biguint().unwrap(), self.pow_2) *
        num::pow(3u.to_biguint().unwrap(), self.pow_3) *
        num::pow(5u.to_biguint().unwrap(), self.pow_5))
    }
}

impl HammingTriple {
    fn new(pow_2: uint, pow_3: uint, pow_5: uint) -> HammingTriple {
        HammingTriple {
            pow_2: pow_2,
            pow_3: pow_3,
            pow_5: pow_5,
            ln: (pow_2 as f64) * LN_2 + (pow_3 as f64) * LN_3 + (pow_5 as f64) * LN_5
        }
    }
}

impl Clone for HammingTriple {
    /// Return a deep copy of the value.
    #[inline]
    fn clone(&self) -> HammingTriple { *self }
}

impl PartialEq for HammingTriple {
    fn eq(&self, other: &HammingTriple) -> bool {
        self.pow_2 == other.pow_2 &&
        self.pow_3 == other.pow_3 &&
        self.pow_5 == other.pow_5
    }
}

impl Eq for HammingTriple {}

impl PartialOrd for HammingTriple {
    fn partial_cmp(&self, other: &HammingTriple) -> Option<Ordering> {
        if self == other { Some(Equal) }
        else if self.pow_2 >= other.pow_2 && self.pow_3 >= other.pow_3 &&
            self.pow_5 >= other.pow_5 { Some(Greater) }
        else if self.pow_2 <= other.pow_2 && self.pow_3 <= other.pow_3 &&
            self.pow_5 <= other.pow_5 { Some(Less) }
        else if self.ln > other.ln { Some(Greater) }
        else if self.ln < other.ln { Some(Less) }
        else { None }
    }
}

impl Ord for HammingTriple {
    fn cmp(&self, other: &HammingTriple) -> Ordering {
        // as a last resort we need to calculate the BigUint values and compare them.
        // This should be rare. The reason is that for very big values floating point precision
        // could make hamming_1.ln == hamming_2.ln even if the two numbers are actually different
        self.partial_cmp(other).unwrap_or_else( ||
            self.to_biguint().unwrap().cmp(&other.to_biguint().unwrap())
        )
    }
}

#[test]
fn hamming_iter() {
    let mut hamming = Hamming::<HammingTriple>::new(20);
    assert!(hamming.nth(19).unwrap().to_biguint() == 36u.to_biguint());
}