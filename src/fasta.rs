// http://rosettacode.org/wiki/FASTA_format
// fasta reader in Rust 0.11-pre
// ported from rosettacode D example

use std::path::Path;
use std::io::fs::File;
use std::io::BufferedReader;
use std::strbuf::StrBuf;


// Best to use type parameter <R:Reader> when we pass in BufferedReader
fn format_fasta<R:Reader>(reader: &mut BufferedReader<R>) -> StrBuf {

  let mut first = true;

  let mut res = StrBuf::new();

  for line in reader.lines() {
    let ln = line.unwrap_or(~"<Error: Cannot read line>");
    if ln.slice(0,1) == ">" {
      if first {
        first = false;
      } else {
        res.push_str("\n");
      }
      let s1 = ln.as_slice().slice_from(1).trim() + ": ";
      res.push_str(s1);
    } else {
      let s2 = ln.as_slice().trim();
      res.push_str(s2);
    }
  }
  res
}

fn read_file() -> StrBuf {
  let f = File::open(&Path::new("resources/test_data.fasta"));
  let mut reader = BufferedReader::new(f);
  format_fasta(&mut reader)
}

#[cfg(not(test))]
fn main() {
  let s = read_file();
  println!("{}", s);
}


#[test]
fn test_format_fasta() {
  let s = read_file();
  assert_eq!(s.as_slice(), "Rosetta_Example_1: THERECANBENOSPACE\nRosetta_Example_2: THERECANBESEVERALLINESBUTTHEYALLMUSTBECONCATENATED");
}
