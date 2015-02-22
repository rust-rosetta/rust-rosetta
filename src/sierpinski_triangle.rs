// http://rosettacode.org/wiki/Sierpinski_triangle
#![feature(core)]
use std::iter::{repeat, range_step};

fn main() {
    let order = 4;
    let height = 1 << order;
    let mut state: Vec<bool> = repeat(true).take(height + 1).collect();

    // Compute the triangle line-by-line by viewing it as Pascal's triangle (mod 2)
    for i in (0..height) {
        for _ in (0..height - i - 1) {
            print!(" ");
        }

        for j in (0..i + 1) {
            print!(" {}", if state[j] { "*" } else { " " });
        }

        // Compute the next line
        for j in range_step(i as i32, 0, -1) {
            state[j as usize] ^= state[(j - 1) as usize];
        }

        print!("\n");
    }
}
