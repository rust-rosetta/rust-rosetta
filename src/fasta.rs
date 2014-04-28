// fasta reader in Rust 0.11-pre
// ported from rosettacode D example

use std::path::Path;
use std::io::fs::File;
use std::io::BufferedReader;


fn main() {

  let mut first = true;

  let f = File::open(&Path::new("data.fasta"));
  let mut reader = BufferedReader::new(f);

  for line in reader.lines() {
    let ln = line.unwrap_or(~"<Error: Cannot read line>");
    if ln.slice(0,1) == ">" {
      if first {
        first = false;
      } else {
        println!("");
      }
      print!("{}: ", ln.as_slice().slice_from(1).trim());
    } else {
      print!("{}", ln.as_slice().trim());
    }
  }

  println!("");

}
