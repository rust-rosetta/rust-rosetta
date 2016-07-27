extern crate num;

use std::cmp::min;
use num::{BigUint, Zero, One};

pub struct Solver {
    /// The `cache` is a private implementation detail,
    /// it would be an improvement to throw away unused values
    /// from the cache (to reduce memory for larger inputs)
    cache: Vec<Vec<BigUint>>,
}

impl Solver {
    pub fn new() -> Solver {
        // Setup the cache with the initial row
        Solver { cache: vec![vec![One::one()]] }
    }

    /// Returns a string representing a line
    pub fn row_string(&mut self, idx: usize) -> String {
        let r = self.cumulative(idx);

        (0..idx)
            .map(|i| &r[i + 1] - &r[i])
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }

    /// Convenience method to access the last column in a culmulated calculation
    pub fn row_sum(&mut self, idx: usize) -> &BigUint {
        // This can never fail as we always add zero or one, so it's never empty.
        self.cumulative(idx).last().unwrap()
    }

    fn cumulative(&mut self, idx: usize) -> &[BigUint] {
        for l in self.cache.len()..idx + 1 {
            let mut r: Vec<BigUint> = vec![Zero::zero()];

            for x in 1..l + 1 {
                let w = {
                    let y = &r[x - 1];
                    let z = &self.cache[l - x][min(x, l - x)];
                    y + z
                };
                r.push(w)
            }
            self.cache.push(r);
        }

        &self.cache[idx][..]
    }
}

impl Default for Solver {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let mut solver = Solver::new();

    println!("rows");
    for n in 1..11 {
        println!("{}: {}", n, solver.row_string(n));
    }

    println!("sums");
    for &y in &[23, 123, 1234, 12345] {
        println!("{}: {}", y, solver.row_sum(y));
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;
    use num::BigUint;

    #[test]
    fn cumulative() {
        let mut solver = Solver::new();
        let mut t = |n: usize, expected: &str| {
            assert_eq!(solver.row_sum(n), &expected.parse::<BigUint>().unwrap());
        };

        t(23, "1255");
        t(123, "2552338241");
        t(1234, "156978797223733228787865722354959930");
    }

    #[test]
    fn row() {
        let mut solver = Solver::new();
        let mut t = |n: usize, expected: &str| {
            assert_eq!(solver.row_string(n), expected);
        };

        t(1, "1");
        t(2, "1, 1");
        t(3, "1, 1, 1");
        t(4, "1, 2, 1, 1");
        t(5, "1, 2, 2, 1, 1");
        t(6, "1, 3, 3, 2, 1, 1");
    }
}
