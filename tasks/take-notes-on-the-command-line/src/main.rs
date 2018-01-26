extern crate chrono;

use std::fs::OpenOptions;
use std::io::{self, BufReader, BufWriter};
use std::io::prelude::*;
use std::env;

const FILENAME: &str = "NOTES.TXT";

fn show_notes() -> Result<(), io::Error> {
    let file = OpenOptions::new()
        .read(true)
        .create(true) // create the file if not found
        .write(true) // necessary to create the file
        .open(FILENAME)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}

fn add_to_notes(note: &str) -> Result<(), io::Error> {
    let file = OpenOptions::new()
        .append(true) // disables overwriting, writes to the end of the file
        .create(true)
        .open(FILENAME)?;
    let mut buf_writer = BufWriter::new(file);

    let date_and_time = chrono::Local::now();
    writeln!(buf_writer, "{}", date_and_time)?;

    writeln!(buf_writer, "\t{}", note)
}

fn main() {
    let note = env::args().skip(1).collect::<Vec<_>>();

    if note.is_empty() {
        show_notes().expect("failed to print NOTES.TXT");
    } else {
        add_to_notes(&note.join(" ")).expect("failed to write to NOTES.TXT");
    }
}
