// http://rosettacode.org/wiki/FASTA_format
// Ported and adapted from rosettacode D example
#![feature(old_io)]
#![feature(old_path)]

use std::old_io::fs::File;
use std::old_io::BufferedReader;

// We use a type parameter bound `<T: Buffer>` to accept all kinds of buffers
fn format_fasta<T: Buffer>(reader: &mut T) -> String {
    reader.lines().map(|l| l.unwrap()).fold(String::new(), |mut out, line| {
        // We need to trim new lines
        let ln = line.trim();

        // Lines that begin with '>' require special treatment
        match &ln[..1] {
            ">" => {
                if out.len() > 0 {
                    out.push('\n');
                }

                // Push skipping the '>'
                out.push_str(&ln[1..]);
                out.push_str(": ");
            }
            // Other lines are just pushed
            _ => out.push_str(ln)
        }
        out
    })
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
    assert_eq!(s, "Rosetta_Example_1: THERECANBENOSPACE
Rosetta_Example_2: THERECANBESEVERALLINESBUTTHEYALLMUSTBECONCATENATED");
}
