// http://rosettacode.org/wiki/FASTA_format
// Fasta reader in Rust 0.11-pre
// Ported and adapted from rosettacode D example

use std::path::Path;
use std::io::fs::File;
use std::io::BufferedReader;
use std::strbuf::StrBuf;

// Best to use type parameter <T: Buffer> to accept all kinds of buffers
fn format_fasta<T: Buffer>(reader: &mut T) -> ~str {
    let mut result = StrBuf::new();

    for line in reader.lines() {
        // Using the same name for the next variable will just shadow the previous one
        let ln = line.unwrap();
        
        // We need to trim new lines
        let ln = ln.as_slice().trim();
        
        // Lines that begin with '>' require special treatment
        if ln.slice(0,1) == ">" {
            if result.len() > 0 {
                result.push_str("\n");
            }
            
            // Push skipping the '>'
            result.push_str(ln.slice_from(1) + ": ");
        }
        
        // Other lines are just pushed
        else {
            result.push_str(ln);
        }
    }
    result.to_str()
}

fn read_file() -> ~str {
    let file = File::open(&Path::new("resources/test_data.fasta"));
    format_fasta(&mut BufferedReader::new(file))
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
