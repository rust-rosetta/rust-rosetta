// Implements http://rosettacode.org/wiki/100_doors

// this is the unoptimized version that performs all 100
// passes, as per the original description of the problem

use std::iter::range_inclusive;

#[cfg(not(test))]
fn main() {
    let mut drs = Doors:: New();
    drs.solve();
    println!("{}", drs.states);
}

// states for the 100 doors
// uses a vector of booleans, 
// where state==false means the door is closed
struct Doors {
    pub states: Vec<bool>
 }

impl Doors {
    // creates the first 100 doors, all closed
    fn New() -> Doors {
        Doors{states: Vec::from_fn(100, |_| false) }
    }
    
    // unoptimized solution for the 100 Doors problem, 
    // performs all 100 passes and mutates the vector with
    // the states in place
    fn solve(&mut self)  {
        for pass in range_inclusive(1u, 100u) {
            for door in range_inclusive(1u, 100u) {
                if (door % pass) == 0 {
                    // flip the state of the door
                    *self.states.get_mut(door-1) = !(*self.states.get(door-1)) 
                }
            }           
         }
    }
}

#[test]
fn init_doors() {
    let drs = Doors :: New();
    // check that we have 100 doors...
    assert!(drs.states.len() == 100);
    // ...and they are initially all closed (state==false)
    for i in range_inclusive(0u,99u) {
        assert!(*drs.states.get(i) == false);
    }    
}

#[test]
fn solution() {
    let mut drs = Doors:: New();
    drs.solve();
    
    // test that the doors with index corresponding to 
    // a perfect square are now open
    for i in range_inclusive(1u,10u) {
        assert!(drs.states.get(i*i - 1));        
    }
}