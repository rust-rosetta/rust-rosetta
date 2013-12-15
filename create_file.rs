// Implements http://rosettacode.org/wiki/Create_a_file

use std::io::{File, fs};
use std::libc::S_IRWXU;

fn main () {
  // Create a new file.
  let mut new_file = File::create(&Path::new("output.txt")).unwrap();
  // Write something trivial to it.
  write!(&mut new_file as &mut Writer, "Nothing here...");
  // Create a directory.
  fs::mkdir(&Path::new("docs"), S_IRWXU as u32)
}
