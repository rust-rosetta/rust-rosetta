#![feature(iterator_step_by)]

use std::fmt;

enum Direction {
    RIGHT,
    UP,
    LEFT,
    DOWN,
}
use Direction::*;

fn is_prime(a: u32) -> bool {
    match a {
        2 => true,
        x if x <= 1 || x % 2 == 0 => false,
        _ => {
            let max = f64::sqrt(a as f64) as u32;
            for x in (3..max).step_by(2) {
                if a % x == 0 {
                    return false;
                }
            }
            true
        }
    }
}

struct Ulam {
    u: Vec<Vec<String>>,
}

impl fmt::Display for Ulam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.u {
            write!(f,
                   "{}\n",
                   format!("{:?}", row).replace("\"", "").replace(", ", ""))
                .unwrap();
        }
        write!(f, "\n")
    }
}

#[cfg_attr(feature = "clippy", allow(many_single_char_names))]
fn generate(n: u32, s: u32, c: char) -> Ulam {
    let mut spiral = vec![vec!["".to_string(); n as usize]; n as usize];
    let mut dir = RIGHT;
    let mut y = (n / 2) as usize;
    let mut x = if n % 2 == 0 {
        y - 1
    } else {
        y
    }; // shift left for even n's
    for j in s..n * n + s {
        spiral[y][x] = if is_prime(j) {
            if c == '\0' {
                format!("{:4}", j)
            } else {
                format!("  {} ", c)
            }
        } else {
            " ---".to_string()
        };

        match dir {
            RIGHT => {
                if x as u32 <= n - 1 && spiral[y - 1][x].is_empty() && j > s {
                    dir = UP;
                }
            }
            UP => {
                if spiral[y][x - 1].is_empty() {
                    dir = LEFT;
                }
            }
            LEFT => {
                if x == 0 || spiral[y + 1][x].is_empty() {
                    dir = DOWN;
                }
            }
            DOWN => {
                if spiral[y][x + 1].is_empty() {
                    dir = RIGHT;
                }
            }
        };

        match dir {
            RIGHT => x += 1,
            UP => y -= 1,
            LEFT => x -= 1,
            DOWN => y += 1,
        };
    }
    Ulam { u: spiral }
}

// Program entry point.
fn main() {
    print!("{}", generate(9, 1, '\0'));
    print!("{}", generate(9, 1, '*'));
}
