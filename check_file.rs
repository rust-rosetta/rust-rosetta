// Implements http://rosettacode.org/wiki/Check_that_file_exists
use std::io;
use std::io::fs;

fn main() {
  let paths = ["input.txt", "docs"];
  paths.map(|&x| check_file(x));
}

fn check_file(i: &'static str) -> () {
  let check = Path::new(i);
  match io::result(|| fs::stat(&check)) {
    Ok(_) => println!("{} exists.", check.display()),
    Err(_)   => println!("{} does not exist.", check.display())
  };
}
