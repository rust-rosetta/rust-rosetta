// Implements http://rosettacode.org/wiki/100_doors
use std::num::Float;
use std::iter::{Map, RangeInclusive, range_inclusive};

type DoorIter = Map<u32, DoorState, RangeInclusive<u32>, fn(u32) -> DoorState>;

#[derive(Show, PartialEq)]
enum DoorState { Open, Closed, }

// This is an example of returning an iterator, this allows the caller to
// choose if they want to allocate or just process as a stream.
fn calculate_doors() -> DoorIter {
    fn door_status(door_number: u32) -> DoorState {
        let x = (door_number as f64).sqrt();
        if x == x.round() { DoorState::Open } else { DoorState::Closed }
    }

    range_inclusive(1u32, 100).map(door_status as fn(u32) -> DoorState)
}

#[cfg(not(test))]
fn main() {
    let doors = calculate_doors();
    for (i, x) in doors.enumerate() { println!("Door {} is {}" , i + 1 , x); }
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
