// Implements http://rosettacode.org/wiki/100_doors

use std::num::Float;
use std::iter::{Map, RangeInclusive, range_inclusive};

type DoorIter<'a> = Map<'a, i32, DoorState, RangeInclusive<i32>>;

#[deriving(Show, PartialEq)]
enum DoorState {
    Open,
    Closed
}

// This is an example of returning an iterator, this allows the caller to
// choose if they want to allocate or just process as a stream.
fn calculate_doors<'a>() -> DoorIter<'a> {
    range_inclusive(1, 100).map(|f| {
        let x = (f as f64).sqrt();
        if x == x.round() {DoorState::Open} else {DoorState::Closed}
    })
}

#[cfg(not(test))]
fn main() {
    let doors = calculate_doors();
    for (i, x) in doors.enumerate() {
        println!("Door {} is {}", i + 1, x);
    }
}

#[test]
fn solution() {
    let doors = calculate_doors().collect::<Vec<DoorState>>();

    // test that the doors with index corresponding to
    // a perfect square are now open
    for i in range_inclusive(1u,10u) {
        assert_eq!(doors[i*i - 1], DoorState::Open);
    }
}
