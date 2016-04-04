// http://rosettacode.org/wiki/Dot_product
extern crate num;

use num::traits::Zero;
use std::ops::Mul;

fn dotp<T: Zero + Mul<Output = T> + Copy>(this: &[T], other: &[T]) -> T {
    assert!(this.len() == other.len(), "The dimensions must be equal");

    let zero: T = Zero::zero();
    this.iter()
        .zip(other.iter())
        .map(|(&a, &b)| a * b)
        .fold(zero, |sum, n| sum + n)
}

fn main() {
    let a = &[1.0f32, 3.0, -5.0];
    let b = &[4.0f32, -2.0, -1.0];
    println!("{}", dotp(a, b));
}

#[test]
fn test_dotp() {
    let result = dotp(&[1i32, 3, -5], &[4i32, -2, -1]);
    assert_eq!(result, 3);
}
