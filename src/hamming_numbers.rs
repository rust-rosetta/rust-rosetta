// Implements http://rosettacode.org/wiki/Hamming_numbers
// port of one of the scala solutions
extern crate num;
use num::bigint::BigUint;
use std::cmp::min;
use std::sync::spsc_queue::Queue;

fn int_to_biguint(i: int) -> BigUint {
    FromPrimitive::from_int(i).unwrap()
}

#[cfg(not(test))]
fn main() {
    let mut hamming = Hamming::new(1691);

    println!("first 20 Hamming numbers")
    for _ in range(0,19) {
        print!("{} ", hamming.next().unwrap());
    }

    println!("\n\n1691st Hamming number");
    println!("{}",hamming.nth(1691-20).unwrap())
}

struct Hamming {
    q2: Queue<BigUint>,
    q3: Queue<BigUint>,
    q5: Queue<BigUint>
}

impl Hamming {
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

    fn enqueue(&self, n: BigUint) {
        self.q2.push(n * int_to_biguint(2));
        self.q3.push(n * int_to_biguint(3));
        self.q5.push(n * int_to_biguint(5));
    }

}

impl Iterator<BigUint> for Hamming {
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