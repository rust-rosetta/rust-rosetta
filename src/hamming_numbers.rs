// Implements http://rosettacode.org/wiki/Hamming_numbers
// Port of one of the scala solutions
extern crate num;

use num::bigint::BigUint;
use std::cmp::min;
use std::num::One;
use std::collections::{RingBuf, Deque};

// Helper function to avoid repeating `FromPrimitive::from_int(i).unwrap()`
// for every BigUint creation.
fn int_to_biguint(i: int) -> BigUint {
    FromPrimitive::from_int(i).unwrap()
}

#[cfg(not(test))]
fn main() {
    let hamming = Hamming::new(100);

    for (idx, h) in hamming.enumerate() {
        match idx + 1 {
            1..20 => print!("{} ", h),
            i @ 1691 | i @ 1000000 => println!("\n{}th number: {}", i, h),
            i if i < 1000000 =>  continue ,
            _ => break
        }
    }
}

/// Hamming numbers are multiples of 2, 3 or 5.
///
/// We keep them on three queues and extract the lowest (leftmost) value from
/// the three queues at each iteration.
pub struct Hamming {
    // Using a RingBuf as a queue, push to the back, pop from the front
    q2: RingBuf<BigUint>,
    q3: RingBuf<BigUint>,
    q5: RingBuf<BigUint>
}

impl Hamming {
    /// Static constructor method
    /// `n` initializes the capacity of the queues
    fn new(n: uint) -> Hamming {
        let mut h = Hamming {
            q2: RingBuf::with_capacity(n),
            q3: RingBuf::with_capacity(n),
            q5: RingBuf::with_capacity(n)
        };

        h.q2.push(One::one());
        h.q3.push(One::one());
        h.q5.push(One::one());

        h
    }

    /// Pushes the next multiple of `n` (x2, x3, x5) to the queues
    fn enqueue(&mut self, n: &BigUint) {
        self.q2.push(n * int_to_biguint(2));
        self.q3.push(n * int_to_biguint(3));
        self.q5.push(n * int_to_biguint(5));
    }
}

// Implements the `Iterator` trait, so we can generate Hamming numbers lazily
impl Iterator<BigUint> for Hamming {
    // The core of the work is done in the `next` method.
    // We check which of the 3 queues has the lowest candidate and extract it
    // as the next Hamming number.
    fn next(&mut self) -> Option<BigUint> {
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
    let mut h = Hamming::new(5);
    h.q2.push(int_to_biguint(1));
    h.q2.push(int_to_biguint(2));
    h.q2.push(int_to_biguint(4));

    assert_eq!(h.q2.pop_front().unwrap(), One::one());
}

#[test]
fn try_enqueue() {
    let mut h = Hamming::new(5);

    h.enqueue(&int_to_biguint(1));
    h.enqueue(&int_to_biguint(2));
    h.enqueue(&int_to_biguint(3));

    assert_eq!(h.q2.pop_front().unwrap(), int_to_biguint(1));
    assert_eq!(h.q3.pop_front().unwrap(), int_to_biguint(1));
    assert_eq!(h.q5.pop_front().unwrap(), int_to_biguint(1));
    assert_eq!(h.q2.pop_front().unwrap(), int_to_biguint(2));
    assert_eq!(h.q3.pop_front().unwrap(), int_to_biguint(3));
    assert_eq!(h.q5.pop_front().unwrap(), int_to_biguint(5));
 }

#[test]
fn hamming_iter() {
    let mut hamming = Hamming::new(20);
    assert_eq!(hamming.nth(19), Some(int_to_biguint(36)));
}
