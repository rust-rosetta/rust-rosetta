// Implements http://rosettacode.org/wiki/Generic_swap
use std::util::swap;

struct Gizmo(&'static str);
//struct Waffle(&'static str);

fn main() {
  println!("Same type:");
  let mut thing_one = Gizmo("Bear");
  let mut thing_two = Gizmo("Moose");
  println!("Thing 1: {:?}, Thing 2: {:?}", thing_one, thing_two);
  swap(&mut thing_one, &mut thing_two);
  println!("Thing 1: {:?}, Thing 2: {:?}", thing_one, thing_two);

  /*
  println!("Differing types:");
  let mut thing_one = Gizmo("Bear");
  let mut thing_two = Waffle("Moose");
  println!("Thing 1: {:?}, Thing 2: {:?}", thing_one, thing_two);
  swap(&mut thing_one, &mut thing_two);
  println!("Thing 1: {:?}, Thing 2: {:?}", thing_one, thing_two);
  */
}
