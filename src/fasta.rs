// http://rosettacode.org/wiki/FASTA_format
// Fasta reader in Rust 0.11-pre
// Ported and adapted from rosettacode D example

use std::path::Path;
use std::io::fs::File;
use std::io::BufferedReader;
use std::string::String;

// Best to use type parameter <T: Buffer> to accept all kinds of buffers
fn format_fasta<T: Buffer>(reader: &mut T) -> String {
    let mut result = String::new();

    for line in reader.lines() {
        // Using the same name for the next variable will just shadow the previous one
        let ln = line.unwrap();

        // We need to trim new lines
        let ln = ln.as_slice().trim();

        // Lines that begin with '>' require special treatment
        if ln.slice(0,1) == ">" {
            if result.len() > 0 {
                result.push_char('\n');
            }

            // Push skipping the '>'
            result.push_str(ln.slice_from(1));
            result.push_str(": ");
        } else {
            // Other lines are just pushed
            result.push_str(ln);
        }
    }

    result
}

fn read_file() -> String {
    let file = File::open(&Path::new("src/resources/test_data.fasta"));
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
    assert_eq!(s.as_slice(), "Rosetta_Example_1: THERECANBENOSPACE
Rosetta_Example_2: THERECANBESEVERALLINESBUTTHEYALLMUSTBECONCATENATED");
}
