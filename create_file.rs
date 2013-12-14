// Implements http://rosettacode.org/wiki/Create_a_file

use std::io::File;

fn main () {
  // Create a new file.
  let mut new_file = File::create(&Path::new("output.txt")).unwrap();
  // Write something trivial to it.
  new_file.write(bytes!("Nothing here..."))
}
