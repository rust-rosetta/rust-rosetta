// http://rosettacode.org/wiki/Hamming_numbers
extern crate num;
use num::bigint::{BigUint, ToBigUint};
use std::cmp::min;
use std::num::{One, one};
use std::collections::{RingBuf, Deque};

// needed because hamming_numbers_alt uses this as a library
#[allow(dead_code)]
#[cfg(not(test))]
fn main() {
    // capacity of the queue currently needs to be a power of 2 because of a bug with RingBuf
    let hamming : Hamming<BigUint> = Hamming::new(128);

    for (idx, h) in hamming.enumerate().take(1_000_000) {
        match idx + 1 {
            1...20 => print!("{} ", h.to_biguint().unwrap()),
            i @ 1691 | i @ 1000000 => println!("\n{}th number: {}", i, h.to_biguint().unwrap()),
            _ =>  continue
        }
    }
}
//representing a Hamming number as a BigUint
impl HammingNumber for BigUint {
    // returns the multipliers 2, 3 and 5 in the representation for the HammingNumber
    fn multipliers()-> (BigUint, BigUint, BigUint) {
        (2u.to_biguint().unwrap(),
        3u.to_biguint().unwrap(),
        5u.to_biguint().unwrap())
    }
}

/// representation of a Hamming number
/// allows to abstract on how the hamming number is stored
/// i.e. as BigUint directly or just as the powers of 2, 3 and 5 used to build it
pub trait HammingNumber : Eq + Ord + ToBigUint + Mul<Self, Self> + One {
    fn multipliers() -> (Self, Self, Self);
}

/// Hamming numbers are multiples of 2, 3 or 5.
///
/// We keep them on three queues and extract the lowest (leftmost) value from
/// the three queues at each iteration.
pub struct Hamming<T> {
    // Using a RingBuf as a queue, push to the back, pop from the front
    q2: RingBuf<T>,
    q3: RingBuf<T>,
    q5: RingBuf<T>
}

impl<T: HammingNumber> Hamming<T> {
    /// Static constructor method
    /// `n` initializes the capacity of the queues
    pub fn new(n: uint) -> Hamming<T> {
        let mut h = Hamming {
            q2: RingBuf::with_capacity(n),
            q3: RingBuf::with_capacity(n),
            q5: RingBuf::with_capacity(n)
        };

        h.q2.push(one());
        h.q3.push(one());
        h.q5.push(one());

        h
    }

    /// Pushes the next multiple of `n` (x2, x3, x5) to the queues
    pub fn enqueue(&mut self, n: &T) {
        let (two, three, five) = HammingNumber::multipliers();
        self.q2.push(*n * two);
        self.q3.push(*n * three);
        self.q5.push(*n * five);
    }
}

// Implements the `Iterator` trait, so we can generate Hamming numbers lazily
impl<T: HammingNumber> Iterator<T> for Hamming<T> {
    // The core of the work is done in the `next` method.
    // We check which of the 3 queues has the lowest candidate and extract it
    // as the next Hamming number.
    fn next(&mut self) -> Option<T> {
        // Return `pop_targets` so the borrow from `front()` will be finished
        let (two, three, five) = match (self.q2.front(),
                                        self.q3.front(),
                                        self.q5.front()) {
            (Some(head2), Some(head3), Some(head5)) => {
                let n = min(head2, min(head3, head5));
                (head2 == n, head3 == n, head5 == n)
            },
            _ => unreachable!()
        };

        let h2 = if two { self.q2.pop_front() } else { None };
        let h3 = if three { self.q3.pop_front() } else { None };
        let h5 = if five { self.q5.pop_front() } else { None };

        match h2.or(h3).or(h5) {
            Some(n) => {
                self.enqueue(&n);
                Some(n)
            }
            None => unreachable!()
        }
    }
}

#[test]
fn create() {
    let mut h = Hamming::<BigUint>::new(5);
    h.q2.push(one::<BigUint>());
    h.q2.push(one::<BigUint>() * 3u.to_biguint().unwrap());

    assert_eq!(h.q2.pop_front().unwrap(), one::<BigUint>());
}

#[test]
fn try_enqueue() {
    let mut h = Hamming::<BigUint>::new(5);
    let (two, three, five) = HammingNumber::multipliers();
    h.enqueue(&one::<BigUint>());
    h.enqueue(&(&one::<BigUint>() * two));

    assert!(h.q2.pop_front().unwrap() == one::<BigUint>());
    assert!(h.q3.pop_front().unwrap() == one::<BigUint>());
    assert!(h.q5.pop_front().unwrap() == one::<BigUint>());
    assert!(h.q2.pop_front().unwrap() == one::<BigUint>() * two);
    assert!(h.q3.pop_front().unwrap() == one::<BigUint>() * three);
    assert!(h.q5.pop_front().unwrap() == one::<BigUint>() * five);
 }

#[test]
fn hamming_iter() {
    let mut hamming = Hamming::<BigUint>::new(20);
    assert!(hamming.nth(19).unwrap().to_biguint() == 36u.to_biguint());
}

#[test]
fn hamming_iter_1million() {
    let mut hamming = Hamming::<BigUint>::new(128);
    // one-million-th hamming number has index 999_999 because indexes are zero-based
    assert_eq!(hamming.nth(999_999).unwrap().to_biguint(),
        from_str(
        "519312780448388736089589843750000000000000000000000000000000000000000000000000000000")
        );
}
