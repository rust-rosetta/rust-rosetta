use std::ops::Deref;
use std::fmt;

// primitive gates
fn not(a: bool) -> bool {
    !a
}
fn or(a: bool, b: bool) -> bool {
    a || b
}
fn and(a: bool, b: bool) -> bool {
    a && b
}

/// xor gate [2x not, 2x and, 1x or]
/// (A & !B) | (B & !A)
fn xor(a: bool, b: bool) -> bool {
    or(and(a, not(b)), and(b, not(a)))
}

/// half adder [1x xor, 1x and]
/// S = A ^ B, C = A & B
fn half_adder(a: bool, b: bool) -> (bool, bool) {
    (xor(a, b), and(a, b))
}

/// full adder [2x half adder, 1x or]
/// t = (C0 + A), t2 = t.S + B
/// S = t2.S, C = t.C | t2.C
fn full_adder(a: bool, b: bool, carry: bool) -> (bool, bool) {
    let (s0, c0) = half_adder(carry, a);
    let (s1, c1) = half_adder(s0, b);

    (s1, or(c0, c1))
}

#[derive(Copy, Clone)]
struct Nibble([bool; 4]);
impl Nibble {
    fn new(arr: [u8; 4]) -> Nibble {
        Nibble([arr[0] != 0, arr[1] != 0, arr[2] != 0, arr[3] != 0])
    }

    fn from_u8(n: u8) -> Nibble {
        Nibble::new([n & 8, n & 4, n & 2, n & 1])
    }

    fn to_u8(&self, carry: bool) -> u8 {
        match u8::from_str_radix(&(format!("{}", self))[..], 2) {
            Ok(n) if carry => n + 16,
            Ok(n) => n,
            Err(_) => unreachable!(),
        }
    }
}

impl fmt::Display for Nibble {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f,
               "{}",
               self.iter()
                   .map(|&b| {
                if b {
                    '1'
                } else {
                    '0'
                }
            })
                   .collect::<String>())
    }
}

/// We implement `Deref` so we can index the Nibble easily
impl<'a> Deref for Nibble {
    type Target = [bool; 4];

    fn deref(&self) -> &[bool; 4] {
        let Nibble(ref inner) = *self;
        inner
    }
}

/// 4bit adder [4x full adder]
/// calculate each bit of the sum, propagate the carry
fn four_bit_adder(a: Nibble, b: Nibble, carry: bool) -> (Nibble, bool) {
    let (s0, carry) = full_adder(a[3], b[3], carry);
    let (s1, carry) = full_adder(a[2], b[2], carry);
    let (s2, carry) = full_adder(a[1], b[1], carry);
    let (s3, carry) = full_adder(a[0], b[0], carry);

    (Nibble([s3, s2, s1, s0]), carry)
}

fn main() {
    let nib_a = Nibble::new([1u8, 0, 1, 1]);
    let a = nib_a.to_u8(false);
    let b = 6;
    let nib_b = Nibble::from_u8(b);
    let (result, carry) = four_bit_adder(nib_a, nib_b, false);
    println!("{} + {} = {} | {} + {} = {} | overflow: {}",
             a,
             b,
             result.to_u8(carry),
             nib_a,
             nib_b,
             result,
             carry)
}

#[test]
fn test_not() {
    assert_eq!(true, not(false));
    assert_eq!(false, not(true));
}

#[test]
fn test_or() {
    assert_eq!(false, or(false, false));
    assert_eq!(true, or(true, false));
    assert_eq!(true, or(false, true));
    assert_eq!(true, or(true, true));
}

#[test]
fn test_and() {
    assert_eq!(false, and(false, false));
    assert_eq!(false, and(false, true));
    assert_eq!(false, and(true, false));
    assert_eq!(true, and(true, true));
}

#[test]
fn test_xor() {
    assert_eq!(false, xor(false, false));
    assert_eq!(true, xor(false, true));
    assert_eq!(true, xor(true, false));
    assert_eq!(false, xor(true, true));
}

#[test]
fn test_full_add() {
    assert_eq!((false, false), full_adder(false, false, false));
    assert_eq!((true, false), full_adder(false, false, true));
    assert_eq!((true, false), full_adder(false, true, false));
    assert_eq!((true, false), full_adder(true, false, false));
    assert_eq!((false, true), full_adder(false, true, true));
    assert_eq!((false, true), full_adder(true, false, true));
    assert_eq!((false, true), full_adder(true, true, false));
    assert_eq!((true, true), full_adder(true, true, true));
}

#[test]
fn test_four_bit_adder() {
    for (a, b) in (0..std::u8::MAX).map(|n| (n >> 4, n & 15)) {
        let nib_a = Nibble::from_u8(a);
        let nib_b = Nibble::from_u8(b);

        let (result, carry) = four_bit_adder(nib_a, nib_b, false);
        assert_eq!(a + b, result.to_u8(carry));
        let (result, carry) = four_bit_adder(nib_a, nib_b, true);
        assert_eq!(a + b + 1, result.to_u8(carry));
    }
}
