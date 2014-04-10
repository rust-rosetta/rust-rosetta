// Implements http://rosettacode.org/wiki/Create_a_file

extern crate libc;

use std::io::{File, fs};
use libc::S_IRWXU;

fn main () {
  // Create a new file.
  let mut new_file = File::create(&Path::new("output.txt")).unwrap();
  // Write something trivial to it.
  match writeln!(&mut new_file as &mut Writer, "Nothing here...") {
      Ok(()) => (),
      Err(e) => println!("failed to write to file: {}", e),
  }
  // Create a directory.
  fs::mkdir(&Path::new("docs"), S_IRWXU as u32).ok();
}
