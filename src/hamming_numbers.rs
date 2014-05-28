// Implements http://rosettacode.org/wiki/Hamming_numbers
// port of one of the scala solutions
extern crate num;
use num::bigint::BigUint;
use std::cmp::min;
use std::sync::spsc_queue::Queue;

// helper function to avoid repeating
// FromPrimitive::from_int(i).unwrap()
// for every BigUint creation
fn int_to_biguint(i: int) -> BigUint {
    FromPrimitive::from_int(i).unwrap()
}

#[cfg(not(test))]
fn main() {
    let mut hamming = Hamming::new(1691);

    println!("first 20 Hamming numbers")
    for _ in range(0,20) {
        print!("{} ", hamming.next().unwrap());
    }

    println!("\n\n1691st Hamming number");
    // we've already iterated over the first 20 numbers
    // so to get at number 1691 we have to subtract 20
    println!("{}",hamming.nth(1691-20).unwrap())
}

// Hamming numbers are multiples
// of 2, 3 or 5. We keep them on three
// queues and extract the lowest (leftmost) value
// from the three queues at each iteration
struct Hamming {
    q2: Queue<BigUint>,
    q3: Queue<BigUint>,
    q5: Queue<BigUint>
}

impl Hamming {
    // constructor method
    // n initializes the capacity of the queues
    fn new(n: uint) -> Hamming {
        let h = Hamming {
            q2: Queue::new(n),
            q3: Queue::new(n),
            q5: Queue::new(n)
        };

        h.q2.push(int_to_biguint(1));
        h.q3.push(int_to_biguint(1));
        h.q5.push(int_to_biguint(1));

        h
    }

    // adds the next multiple (x2, x3, x5)
    // to the queues
    fn enqueue(&self, n: BigUint) {
        self.q2.push(n * int_to_biguint(2));
        self.q3.push(n * int_to_biguint(3));
        self.q5.push(n * int_to_biguint(5));
    }
}

// implements an Iterator, so we
// can extract Hamming numbers more easily
impl Iterator<BigUint> for Hamming {
    // the core of the work is done in the next method.
    // We check which of the 3 queues has the lowest
    // candidate and extract it as the next Hamming number
    fn next(&mut self) -> Option<BigUint> {
        let (head2, head3, head5) =
            (   self.q2.peek().unwrap(),
                self.q3.peek().unwrap(),
                self.q5.peek().unwrap());

        let n = min(head2.clone(), min(head3.clone(), head5.clone()));

        if *head2 == n {self.q2.pop();}
        if *head3 == n {self.q3.pop();}
        if *head5 == n {self.q5.pop();}

        self.enqueue(n.clone());
        Some(n)
    }
}

#[test]
fn create() {
    let h = Hamming::new(5);
    h.q2.push(int_to_biguint(1));
    h.q2.push(int_to_biguint(2));
    h.q2.push(int_to_biguint(4));

    let _ = h.q2.peek();
    assert!(h.q2.pop().unwrap() == int_to_biguint(1));
}

#[test]
fn try_enqueue() {
    let h = Hamming::new(5);
    h.enqueue(int_to_biguint(1));
    h.enqueue(int_to_biguint(2));
    h.enqueue(int_to_biguint(3));

    assert!(h.q2.pop().unwrap() == int_to_biguint(1));
    assert!(h.q3.pop().unwrap() == int_to_biguint(1));
    assert!(h.q5.pop().unwrap() == int_to_biguint(1));
    assert!(h.q2.pop().unwrap() == int_to_biguint(2));
    assert!(h.q3.pop().unwrap() == int_to_biguint(3));
    assert!(h.q5.pop().unwrap() == int_to_biguint(5));
 }

#[test]
fn hamming_iter() {
    let mut hamming = Hamming::new(20);
    assert!(hamming.nth(19).unwrap() == int_to_biguint(36));
}