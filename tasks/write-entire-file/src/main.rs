use std::io::prelude::*;
use std::fs::OpenOptions;

fn main() {
    let contents = r"(Over)write a file so that it contains a string.

The reverse of Read entire file-for when you want to update or
create a file which you would read in its entirety all at once.";

    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("output.txt")
        .unwrap();
    output.write_all(contents.as_bytes()).unwrap();
}
