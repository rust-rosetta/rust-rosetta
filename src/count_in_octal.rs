// Implements http://rosettacode.org/wiki/Count_in_octal
#![feature(core)]

use std::u8;
use std::iter::range_inclusive;

fn main() {
    // We count from 0 to 255 (377 in octal)
      for i in range_inclusive(0, u8::MAX) {
            println!("{:o}", i);
      }
}
