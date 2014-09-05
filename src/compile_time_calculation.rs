// Implements http://rosettacode.org/wiki/Compile-time_calculation

#![feature(macro_rules)]

macro_rules! factorial(
  ($inp:expr) => (
    range(1u, 1 + $inp).fold(1, |accum, elem| accum * elem);
  );
)

#[cfg(not(test))]
fn main() {
	println!("{:u}", factorial!(10u));
}

#[test]
fn test_tenfactorial() {
  let tenfactorial: uint = factorial!(10u);
  assert_eq!(tenfactorial, 3628800);
}