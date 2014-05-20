// Implements http://rosettacode.org/wiki/Dot_product

use std::ops::{Add,Mul};
use std::num::Zero;

fn dotp<'a,T:Add<T,T>+Mul<T,T>+Zero+Copy>(this:&'a [T], other:&'a [T]) -> T {
  if this.len() != other.len() {
    fail!("The dimensions must be equal!");
  }
  let zero: T = Zero::zero();
  this.iter().zip(other.iter()).map(|(&a,&b)|a*b).fold(zero,|sum,n|sum+n)
}

#[cfg(not(test))]
fn main() {
  let a = [1.0, 3.0, -5.0];
  let b = [4.0, -2.0, -1.0];
  println!("{}", dotp(a.as_slice(),b.as_slice()));
}

#[test]
fn testDotp() {
  assert_eq!(dotp([1,3,-5].as_slice(),[4,-2,-1].as_slice()),3);
}
