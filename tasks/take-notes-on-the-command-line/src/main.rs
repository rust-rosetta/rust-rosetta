extern crate time;

use std::fs::{OpenOptions};
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::env;

fn show_notes() {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("NOTES.TXT")
        .unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn add_to_notes(note: &str) {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("NOTES.TXT")
        .unwrap();
    let mut buf_writer = BufWriter::new(file);

    let now = time::now();
    let date_and_time = now.asctime();
    writeln!(buf_writer, "{}", date_and_time).unwrap();

    writeln!(buf_writer, "\t{}", note).unwrap();
}

fn main() {
    let note = env::args().skip(1).collect::<Vec<_>>();

    if note.is_empty() {
        show_notes();
    } else {
        add_to_notes(&note.join(" "));
    }
}
