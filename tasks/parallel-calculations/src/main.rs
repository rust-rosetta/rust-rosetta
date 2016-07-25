//! See http://static.rust-lang.org/doc/master/guide-tasks.html for information
//! about tasks, channels, future, etc.

extern crate eventual;

extern crate prime_decomposition;

use std::thread::spawn;
use std::sync::mpsc;

use eventual::{Future, Async};
use prime_decomposition::factor;

/// Returns the minimal prime factor of a number
fn min_factor(x: usize) -> usize {
    // factor returns a sorted vector, so we just take the first element
    factor(x)[0]
}

/// Returns the largest minimal factor of the numbers in a slice
/// The function is implemented using the Future struct from crate "eventual"
pub fn largest_min_factor_fut(numbers: &[usize]) -> usize {
    // We will save the future values of the minimal factor in the results vec
    let results: Vec<Future<usize, ()>> = (0..numbers.len())
        .map(|i| {
            let number = numbers[i];
            Future::spawn(move || min_factor(number))
        })
        .collect();
    // Get the largest minimal factor of all results
    results.into_iter().map(|f| f.await().ok().unwrap()).max().unwrap()
}

/// Returns the largest minimal factor of the numbers in a slice
/// The function is implemented using a channel
pub fn largest_min_factor_chan(numbers: &[usize]) -> usize {
    let (sender, receiver) = mpsc::channel();

    // Send all the minimal factors
    for &x in numbers {
        let child_sender = sender.clone();
        spawn(move || child_sender.send(min_factor(x)).unwrap());
    }

    // Receive them and keep the largest one
    numbers.iter().fold(0, |max, _| std::cmp::max(receiver.recv().unwrap(), max))
}

fn main() {
    // Numbers to be factorized
    let numbers = &[1122725, 1125827, 1122725, 1152800, 1157978, 1099726];

    let max = largest_min_factor_fut(numbers);
    println!("The largest minimal factor is {}", max);
}

#[cfg(test)]
mod tests {
    use super::{largest_min_factor_chan, largest_min_factor_fut};

    /// We don't have benchmarks because the Bencher doesn't work good with tasks
    #[test]
    fn test_basic() {
        let numbers = &[25, 80, 256, 55, 18, 19];
        assert_eq!(largest_min_factor_fut(numbers), 19);
    }

    #[test]
    fn test_equivalence() {
        let numbers = &[1122725, 1125827, 1122725, 1152800, 1157978, 1099726];
        assert_eq!(largest_min_factor_chan(numbers),
                   largest_min_factor_fut(numbers));
    }
}
