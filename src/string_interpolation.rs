// Implements http://rosettacode.org/wiki/String_interpolation

#[cfg(not(test))]
fn main() {
  let original = "Mary had a X lamb";
  let little = "little";
  let replaced = original.replace("X",little);
  println!("{}",replaced);
}
