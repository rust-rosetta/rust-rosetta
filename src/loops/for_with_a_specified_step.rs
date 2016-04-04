// http://rosettacode.org/wiki/Loops/For_with_a_specified_step
#![feature(step_by)]

fn main() {
    for i in (2..8 + 1).step_by(2) {
        print!("{}", i);
    }
    println!("who do we appreciate?!");
}
