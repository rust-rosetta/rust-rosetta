// http://rosettacode.org/wiki/Loops/Downward_for

#![feature(inclusive_range_syntax)]

fn main() {
    for i in (1...10).rev() {
        println!("{}", i);
    }
}
