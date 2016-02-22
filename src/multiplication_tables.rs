// http://rosettacode.org/wiki/Multiplication_tables

#![feature(range_inclusive)]

use std::iter::range_inclusive;

const LIMIT: i32 = 12;

fn main() {
    for i in range_inclusive(1, LIMIT) {
        print!("{:3} ", i);
    }
    print!("\n");

    for i in 0..LIMIT {
        print!("----");
    }
    print!("+\n");

    for i in range_inclusive(1, LIMIT) {
        for j in range_inclusive(1, LIMIT) {
            if j < i {
                print!("    ")
            } else {
                print!("{:3} ", j * i)
            }
        }
        println!("| {}", i);
    }
}
