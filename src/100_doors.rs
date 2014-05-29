// Implements http://rosettacode.org/wiki/100_doors
use std::iter::range_inclusive;

#[cfg(not(test))]
fn main() {
    let drs = doors();
    for (i, x) in drs.iter().enumerate() {
        println!("Door {} is {}", i+1, x);
    }
}

fn doors() -> Vec<DoorState> {
   let mut ret = Vec::with_capacity(100);
   for f in range_inclusive(1f64,100f64) {
        let x = f.sqrt();
        let state = if x == x.round() {Open} else {Closed};
        ret.push(state);
    }
    ret
}

#[deriving(Show, Eq)]
enum DoorState {
    Open,
    Closed
}

#[test]
fn solution() {
    let drs = doors();

    // test that the doors with index corresponding to
    // a perfect square are now open
    for i in range_inclusive(1u,10u) {
        assert_eq!(*drs.get(i*i - 1), Open);
    }
}