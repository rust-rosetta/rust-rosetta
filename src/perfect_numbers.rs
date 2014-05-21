// Implements http://rosettacode.org/wiki/Perfect_numbers

fn perfect_number(n: uint) -> bool {
  let upTo = (n/2)+1;
  range(1,upTo).filter(|&i|n%i==0).fold(0,|s,u|s+u) == n
}

#[cfg(not(test))]
fn main() {
  for n in range(2,10_000u) {
    if perfect_number(n) {
      println!("{}",n);
    }
  }
}

#[test]
fn testFirstFour() {
  let nums = range(2,10_000u).filter(|&n|perfect_number(n)).collect::<Vec<uint>>();
  assert_eq!(nums.as_slice(), &[6,28,496,8128]);
}

#[test]
fn testHighNumber() {
  assert!( perfect_number(33550336) );
}
