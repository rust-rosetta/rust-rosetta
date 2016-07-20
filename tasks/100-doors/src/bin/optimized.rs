extern crate num;

use std::f64;
use std::iter::Map;
use std::ops::Range;

type DoorIter = Map<Range<u32>, fn(u32) -> DoorState>;

#[derive(Debug, PartialEq)]
enum DoorState {
    Open,
    Closed,
}

// This is an example of returning an iterator, this allows the caller to
// choose if they want to allocate or just process as a stream.
fn calculate_doors() -> DoorIter {
    fn door_status(door_number: u32) -> DoorState {
        let x = (door_number as f64).sqrt();
        if (x - x.round()).abs() < f64::EPSILON {
            DoorState::Open
        } else {
            DoorState::Closed
        }
    }

    (1u32..101).map(door_status as fn(u32) -> DoorState)
}

fn main() {
    let doors = calculate_doors();
    for (i, x) in doors.enumerate() {
        println!("Door {} is {:?}", i + 1, x);
    }
}

#[test]
fn solution() {
    let doors = calculate_doors().collect::<Vec<DoorState>>();

    // test that the doors with index corresponding to
    // a perfect square are now open
    for i in 1..11 {
        assert_eq!(doors[i * i - 1], DoorState::Open);
    }
}
