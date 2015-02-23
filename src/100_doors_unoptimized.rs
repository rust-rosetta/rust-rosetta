// Implements http://rosettacode.org/wiki/100_doors
// this is the unoptimized version that performs all 100
// passes, as per the original description of the problem
#![feature(core)]
use std::iter::range_step_inclusive;

#[cfg(not(test))]
fn main() {
    // states for the 100 doors
    // uses a vector of booleans,
    // where state==false means the door is closed
    let mut doors = [false; 100];
    solve(&mut doors);

    for (idx, door) in doors.iter().enumerate() {
        println!("door {} open: {}", idx+1, door);
    }

}

// unoptimized solution for the 100 Doors problem,
// performs all 100 passes and mutates the vector with
// the states in place
fn solve(doors: &mut [bool])  {
    for pass in 1..101 {
        for door in range_step_inclusive(pass, 100, pass) {
            // flip the state of the door
            doors[door-1] = !doors[door-1]
        }
     }
}

#[test]
fn solution() {
    let mut doors = [false;100];
    solve(&mut doors);

    // test that the doors with index corresponding to
    // a perfect square are now open
    for i in 1..11 {
        assert!(doors[i*i - 1]);
    }
}
