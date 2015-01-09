// Implements http://rosettacode.org/wiki/Perfect_numbers
use std::iter::{range_inclusive, AdditiveIterator};

fn perfect_number(n: uint) -> bool {
  range_inclusive(1, n / 2).filter(|&i| n % i == 0).sum() == n
}

#[cfg(not(test))]
fn main() {
  for n in range(2, 10_000u).filter(|&n| perfect_number(n)) {
    println!("{:?}", n);
  }
}

#[test]
fn test_first_four() {
  let nums = range(2, 10_000u).filter(|&n| perfect_number(n))
                              .collect::<Vec<uint>>();
  assert_eq!(nums, [6, 28, 496, 8128]);
}

#[test]
fn test_high_number() {
  assert!( perfect_number(33550336) );
}
