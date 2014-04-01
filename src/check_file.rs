// Implements http://rosettacode.org/wiki/Check_that_file_exists
fn main() {
  let paths = ["input.txt", "docs"];
  paths.iter().map(|&x| check_file(x));
}

fn check_file(i: &'static str) -> () {
  let check = Path::new(i);
  if check.exists() {
    println!("{} exists.", check.display());
  } else {
    println!("{} does not exist.", check.display());
  }
}
