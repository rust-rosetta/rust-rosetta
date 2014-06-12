// Implements http://rosettacode.org/wiki/Short-circuit_evaluation
// not_tested

fn a(x: bool) -> bool {
    println!("Inside function a");
    x
}

fn b(x: bool) -> bool {
    println!("Inside function b");
    x
}

fn main() {
    let booleans = [true, false];

    for &i in booleans.iter() {
        for &j in booleans.iter() {
            println!("{} and {} is {}", i, j, a(i) && b(j));
            println!("{} or {} is {}", i, j, a(i) || b(j));
        }
    }
}
