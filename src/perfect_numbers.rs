// Implements http://rosettacode.org/wiki/Perfect_numbers

fn perfect_number(n: uint) -> bool {
  let limit = (n / 2) + 1;
  let sum = range(1, limit).filter(|&i| n % i == 0)
                           .fold(0, |sum, u| sum + u);
  sum == n
}

#[cfg(not(test))]
fn main() {
  for n in range(2, 10_000u).filter(|&n| perfect_number(n)) {
    println!("{}", n);
  }
}

#[test]
fn test_first_four() {
  let nums = range(2, 10_000u).filter(|&n| perfect_number(n))
                              .collect::<Vec<uint>>();
  assert_eq!(nums.as_slice(), &[6, 28, 496, 8128]);
}

#[test]
fn test_high_number() {
  assert!( perfect_number(33550336) );
}
