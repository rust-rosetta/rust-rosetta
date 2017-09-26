#![feature(iterator_step_by)]

use std::iter::repeat;

fn main() {
    let order = 4;
    let height = 1 << order;
    let mut state: Vec<bool> = repeat(true).take(height + 1).collect();

    // Compute the triangle line-by-line by viewing it as Pascal's triangle (mod 2)
    for i in 0..height {
        for _ in 0..height - i - 1 {
            print!(" ");
        }

        for filled in state.iter().take(i + 1) {
            let fill = if *filled {
                "*"
            } else {
                " "
            };

            print!(" {}", fill);
        }

        // Compute the next line
        for j in (i as i32..0).rev().step_by(1) {
            state[j as usize] ^= state[(j - 1) as usize];
        }

        print!("\n");
    }
}
