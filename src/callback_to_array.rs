// Implements http://rosettacode.org/wiki/Apply_a_callback_to_an_array

fn main () {
  let an_array = [1,2,3,4,5];
  println!("{:?}", an_array);
  println!("{:?}", an_array.iter().map(callback));
}

fn callback(val: &int) -> int {
  val + 1
}
