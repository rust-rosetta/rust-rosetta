extern crate num;

use std::cmp::min;
use std::collections::VecDeque;
use std::ops::Mul;

use num::bigint::{BigUint, ToBigUint};
use num::traits::One;
use num::one;

/// representing a Hamming number as a `BigUint`
impl HammingNumber for BigUint {
    // returns the multipliers 2, 3 and 5 in the representation for the HammingNumber
    fn multipliers() -> (BigUint, BigUint, BigUint) {
        (2u8.to_biguint().unwrap(), 3u8.to_biguint().unwrap(), 5u8.to_biguint().unwrap())
    }
}

/// representation of a Hamming number
/// allows to abstract on how the hamming number is stored
/// i.e. as `BigUint` directly or just as the powers of 2, 3 and 5 used to build it
pub trait HammingNumber: Eq + Ord + ToBigUint + Mul<Output = Self> + One + Clone {
    fn multipliers() -> (Self, Self, Self);
}

/// Hamming numbers are multiples of 2, 3 or 5.
///
/// We keep them on three queues and extract the lowest (leftmost) value from
/// the three queues at each iteration.
pub struct Hamming<T> {
    // Using a VecDeque as a queue, push to the back, pop from the front
    q2: VecDeque<T>,
    q3: VecDeque<T>,
    q5: VecDeque<T>,
}

impl<T: HammingNumber> Hamming<T> {
    /// Static constructor method
    /// `n` initializes the capacity of the queues
    pub fn new(n: usize) -> Hamming<T> {
        let mut h = Hamming {
            q2: VecDeque::with_capacity(n),
            q3: VecDeque::with_capacity(n),
            q5: VecDeque::with_capacity(n),
        };

        h.q2.push_back(one());
        h.q3.push_back(one());
        h.q5.push_back(one());

        h
    }

    /// Pushes the next multiple of `n` (x2, x3, x5) to the queues
    pub fn enqueue(&mut self, n: T) {
        let (two, three, five): (T, T, T) = HammingNumber::multipliers();
        self.q2.push_back(two * n.clone());
        self.q3.push_back(three * n.clone());
        self.q5.push_back(five * n.clone());
    }
}

/// Implements the `Iterator` trait, so we can generate Hamming numbers lazily
impl<T: HammingNumber> Iterator for Hamming<T> {
    type Item = T;

    /// The core of the work is done in the `next` method.
    /// We check which of the 3 queues has the lowest candidate and extract it
    /// as the next Hamming number.
    fn next(&mut self) -> Option<T> {
        // Return `pop_targets` so the borrow from `front()` will be finished
        let (two, three, five) = match (self.q2.front(), self.q3.front(), self.q5.front()) {
            (Some(head2), Some(head3), Some(head5)) => {
                let n = min(head2, min(head3, head5));
                (head2 == n, head3 == n, head5 == n)
            }
            _ => unreachable!(),
        };

        let h2 = if two {
            self.q2.pop_front()
        } else {
            None
        };
        let h3 = if three {
            self.q3.pop_front()
        } else {
            None
        };
        let h5 = if five {
            self.q5.pop_front()
        } else {
            None
        };

        match h2.or(h3).or(h5) {
            Some(n) => {
                self.enqueue(n.clone());
                Some(n)
            }
            None => unreachable!(),
        }
    }
}

#[test]
fn create() {
    let mut h = Hamming::<BigUint>::new(5);
    h.q2.push_back(one::<BigUint>());
    h.q2.push_back(one::<BigUint>() * 3.to_biguint().unwrap());

    assert_eq!(h.q2.pop_front().unwrap(), one::<BigUint>());
}

#[test]
fn try_enqueue() {
    let mut h = Hamming::<BigUint>::new(5);
    let (two, three, five): (BigUint, BigUint, BigUint) = HammingNumber::multipliers();
    h.enqueue(one::<BigUint>());
    h.enqueue((one::<BigUint>() * two.clone()));

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
    assert!(hamming.nth(19).unwrap().to_biguint() == 36.to_biguint());
}

#[ignore]
#[test]
fn hamming_iter_1million() {
    let mut hamming = Hamming::<BigUint>::new(128);
    // one-million-th hamming number has index 999_999 because indexes are zero-based
    let millionth_hamming_number = "51931278044838873608958984375000000000000000000000000000000000\
                                    0000000000000000000000";
    assert_eq!(hamming.nth(999_999).unwrap().to_biguint(),
               millionth_hamming_number.parse::<BigUint>()
                   .ok());
}
