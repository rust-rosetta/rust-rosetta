//! Author: Rahul Sharma
//! Github: https://github.com/creativcoder

use std::io::BufReader;
use std::io::BufWriter;
use std::io::BufRead;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    // opens file for writing replaced lines
    let out_fd = OpenOptions::new()
        .write(true)
        .create(true)
        .open("resources/output.txt");

    // defining a closure write_line
    let write_line = |line: &str| {
        match out_fd {
            Ok(ref v) => {
                let mut writer = BufWriter::new(v);
                writer.write_all(line.as_bytes()).unwrap();
            }
            Err(ref e) => {
                println!("Error:{}", e);
            }
        }
    };
    // read input file
    match File::open("resources/paragraph.txt") {
        Ok(handle) => {
            let mut reader = BufReader::new(handle);
            let mut line = String::new();
            // read the first line
            reader.read_line(&mut line).unwrap();
            // loop until line end
            while line.trim() != "" {
                let mut replaced_line = line.trim().replace("Goodbye London!", "Hello New York!");
                replaced_line = replaced_line + "\n";
                write_line(&replaced_line[..]);
                line.clear();
                reader.read_line(&mut line).unwrap();
            }
        }
        Err(e) => println!("Error:{}", e),
    }
}
