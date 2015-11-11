// http://rosettacode.org/wiki/Sum_of_a_series
#![feature(range_inclusive)]

use std::iter::range_inclusive;

fn main() {
    let sum: f64 = range_inclusive(1u64, 1000).fold(0.,|sum, num| sum + 1./(num*num) as f64);
    println!("{}", sum);
}
