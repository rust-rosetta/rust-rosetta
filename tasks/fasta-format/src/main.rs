//! Ported and adapted from rosettacode D example
use std::fs::File;
use std::io::{BufReader, BufRead};

fn format_fasta<T: BufRead>(reader: &mut T) -> String {
    reader.lines().map(|l| l.unwrap()).fold(String::new(), |mut out, line| {
        // We need to trim new lines
        let ln = line.trim();

        // Lines that begin with '>' require special treatment
        match &ln[..1] {
            ">" => {
                if !out.is_empty() {
                    out.push('\n');
                }

                // Push skipping the '>'
                out.push_str(&ln[1..]);
                out.push_str(": ");
            }
            // Other lines are just pushed
            _ => out.push_str(ln),
        }
        out
    })
}

fn read_file() -> String {
    let file = File::open("resources/test_data.fasta").unwrap();
    format_fasta(&mut BufReader::new(file))
}

fn main() {
    let s = read_file();
    println!("{}", s);
}

#[test]
fn test_format_fasta() {
    let s = read_file();
    assert_eq!(s,
               r"Rosetta_Example_1: THERECANBENOSPACE
Rosetta_Example_2: THERECANBESEVERALLINESBUTTHEYALLMUSTBECONCATENATED");
}
