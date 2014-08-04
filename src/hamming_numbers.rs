extern crate num;
use num::bigint::{BigUint, ToBigUint};
use std::cmp::min;
use std::sync::spsc_queue::Queue;

// needed because hamming_numbers_alt uses this as a library
#[allow(dead_code)]
#[cfg(not(test))]
fn main() {
    let hamming : Hamming<BigUint> = Hamming::new(100);

    for (idx, h) in hamming.enumerate().take(1_000_000) {
        match idx + 1 {
            1..20 => print!("{} ", h.to_biguint().unwrap()),
            i @ 1691 | i @ 1000000 => println!("\n{}th number: {}", i, h.to_biguint().unwrap()),
            _ =>  continue
        }
    }
}
//representing a Hamming number as a BigUint
impl HammingNumber for BigUint {
    fn times_2(&self)-> BigUint {self * 2u.to_biguint().unwrap()}
    fn times_3(&self)-> BigUint {self * 3u.to_biguint().unwrap()}
    fn times_5(&self)-> BigUint {self * 5u.to_biguint().unwrap()}
    fn one() -> BigUint {1u.to_biguint().unwrap()}
}

#[allow(dead_code)]
#[inline(always)]
pub fn one<T: HammingNumber>() -> T { HammingNumber :: one() }

// representation of a Hamming number
// allows to abstract on how the hamming number is stored
// i.e. as BigUint diretly or just as the powers of 2, 3 and 5 used to build it
pub trait HammingNumber : Eq + Ord + Send + ToBigUint {
    fn times_2(&self)-> Self;
    fn times_3(&self)-> Self;
    fn times_5(&self)-> Self;
    fn one() -> Self;
}

// Hamming numbers are multiples
// of 2, 3 or 5. We keep them on three
// queues and extract the lowest (leftmost) value
// from the three queues at each iteration
pub struct Hamming<T> {
    q2: Queue<T>,
    q3: Queue<T>,
    q5: Queue<T>
}

impl<T: HammingNumber> Hamming<T> {
    // constructor method
    // n initializes the capacity of the queues
    pub fn new(n: uint) -> Hamming<T> {
        let h = Hamming {
            q2: unsafe { Queue::new(n) },
            q3: unsafe { Queue::new(n) },
            q5: unsafe { Queue::new(n) }
        };

        h.q2.push(HammingNumber::one());
        h.q3.push(HammingNumber::one());
        h.q5.push(HammingNumber::one());

        h
    }

    // adds the next multiple (x2, x3, x5)
    // to the queues
    pub fn enqueue(&self, n: &T) {
        self.q2.push(n.times_2());
        self.q3.push(n.times_3());
        self.q5.push(n.times_5());
    }
}

// implements an Iterator, so we
// can extract Hamming numbers more easily
impl<T: HammingNumber> Iterator<T> for Hamming<T> {
    // the core of the work is done in the next method.
    // We check which of the 3 queues has the lowest
    // candidate and extract it as the next Hamming number
    fn next(&mut self) -> Option<T> {
        let (head2, head3, head5) =
            ( self.q2.peek().unwrap(),
              self.q3.peek().unwrap(),
              self.q5.peek().unwrap());

        let n = min(&head2, min(&head3, &head5));

        let h2 = { if &head2 == n { self.q2.pop() } else { None} };
        let h3 = { if &head3 == n { self.q3.pop() } else { None} };
        let h5 = { if &head5 == n { self.q5.pop() } else { None} };

        let m = h2.or(h3).or(h5).unwrap();

        self.enqueue(&m);
        Some(m)
    }
}

#[test]
fn create() {
    let h = Hamming::<BigUint>::new(5);
    h.q2.push(one::<BigUint>());
    h.q2.push(one::<BigUint>().times_3());

    let _ = h.q2.peek();
    assert!(h.q2.pop().unwrap() == one::<BigUint>());
}

#[test]
fn try_enqueue() {
    let h = Hamming::<BigUint>::new(5);
    h.enqueue(&one::<BigUint>());
    h.enqueue(&one::<BigUint>().times_2());

    assert!(h.q2.pop().unwrap() == one::<BigUint>());
    assert!(h.q3.pop().unwrap() == one::<BigUint>());
    assert!(h.q5.pop().unwrap() == one::<BigUint>());
    assert!(h.q2.pop().unwrap() == one::<BigUint>().times_2());
    assert!(h.q3.pop().unwrap() == one::<BigUint>().times_3());
    assert!(h.q5.pop().unwrap() == one::<BigUint>().times_5());
 }

#[test]
fn hamming_iter() {
    let mut hamming = Hamming::<BigUint>::new(20);
    assert!(hamming.nth(19).unwrap().to_biguint() == 36u.to_biguint());
}