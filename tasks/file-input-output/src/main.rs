use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    let mut file = File::create("output.txt").unwrap();
    file.write_all(&data).unwrap();
}
