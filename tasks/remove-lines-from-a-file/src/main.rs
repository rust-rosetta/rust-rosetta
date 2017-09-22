// http://rosettacode.org/wiki/Remove_lines_from_a_file

#[macro_use]
extern crate serde_derive;

extern crate docopt;
extern crate serde;

use docopt::Docopt;

use std::io::{BufReader, BufRead};
use std::fs::File;

const USAGE: &'static str = r"
Usage: remove_lines_from_a_file <start> <count> <file>
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_start: usize,
    arg_count: usize,
    arg_file: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let file = BufReader::new(File::open(args.arg_file).unwrap());

    for (i, line) in file.lines().enumerate() {
        let cur = i + 1;

        if cur < args.arg_start || cur >= (args.arg_start + args.arg_count) {
            println!("{}", line.unwrap());
        }
    }
}
