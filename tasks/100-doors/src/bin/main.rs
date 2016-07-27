//! this is the unoptimized version that performs all 100
//! passes, as per the original description of the problem

fn main() {
    // states for the 100 doors
    // uses a vector of booleans,
    // where state==false means the door is closed
    let mut doors = [false; 100];
    solve(&mut doors);

    for (idx, door) in doors.iter().enumerate() {
        println!("door {} open: {}", idx + 1, door);
    }

}

/// unoptimized solution for the 100 Doors problem,
/// performs all 100 passes and mutates the vector with
/// the states in place
fn solve(doors: &mut [bool]) {
    for pass in 1..101 {
        let mut p = pass;
        while p <= 100 {
            // flip the state of the door
            doors[p - 1] = !doors[p - 1];
            p += pass;
        }
    }
}

#[test]
fn solution() {
    let mut doors = [false; 100];
    solve(&mut doors);

    // test that the doors with index corresponding to
    // a perfect square are now open
    for i in 1..11 {
        assert!(doors[i * i - 1]);
    }
}
