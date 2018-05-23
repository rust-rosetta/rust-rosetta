//! This solution uses [rayon](https://github.com/rayon-rs/rayon), a data-parallelism library.
//! Since Rust guarantees that a program has no data races, adding parallelism to a sequential
//! computation is as easy as importing the rayon traits and calling the `par_iter()` method.

extern crate rayon;

extern crate prime_decomposition;

use rayon::prelude::*;

/// Returns the largest minimal factor of the numbers in a slice
pub fn largest_min_factor(numbers: &[usize]) -> usize {
    numbers
        .par_iter()
        .map(|n| {
            // `factor` returns a sorted vector, so we just take the first element.
            prime_decomposition::factor(*n)[0]
        })
        .max()
        .unwrap()
}

fn main() {
    let numbers = &[
        1_122_725, 1_125_827, 1_122_725, 1_152_800, 1_157_978, 1_099_726,
    ];
    let max = largest_min_factor(numbers);
    println!("The largest minimal factor is {}", max);
}

#[cfg(test)]
mod tests {
    use super::largest_min_factor;

    #[test]
    fn test_basic() {
        let numbers = &[25, 80, 256, 55, 18, 19];
        assert_eq!(largest_min_factor(numbers), 19);
    }
}
