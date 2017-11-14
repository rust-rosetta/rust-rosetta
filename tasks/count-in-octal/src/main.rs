#![feature(inclusive_range_syntax)]

use std::u8;

fn main() {
    // We count from 0 to 255 (377 in octal)
    for i in 0..=u8::MAX {
        println!("{:o}", i);
    }
}
