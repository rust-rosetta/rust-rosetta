// Implements http://rosettacode.org/wiki/Averages/Arithmetic_mean
#![feature(core)]

use std::iter::AdditiveIterator;

// The mean is not defined for an empty list, so we must return an Option
fn mean(list: &[f64]) -> Option<f64> {
    match list.len() {
        0 => None,
        n => {
            let sum = list.iter().map(|&x| x).sum();
            Some(sum / n as f64)
        }
    }
}

#[cfg(not(test))]
fn main() {
    let input = [3.0, 1.0, 4.0, 1.0, 5.0, 9.0];

    // This should be 3.833333
    let mean = mean(&input).unwrap();
    println!("{}", mean);
}

#[test]
fn simple_test() {
    let nums = [1.0, 2.0, 3.0, 4.0, 5.0];
    assert_eq!(mean(&nums), Some(3.0));
}

#[test]
fn mean_empty_list() {
    let no_nums = [];
    assert_eq!(mean(&no_nums), None);
}
