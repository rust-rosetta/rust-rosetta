// http://rosettacode.org/wiki/Loops/Downward_for
#![feature(range_inclusive)]
use std::iter::range_inclusive;

fn main() {
    for i in range_inclusive(1, 10).rev() {
        println!("{}", i);
    }
}
