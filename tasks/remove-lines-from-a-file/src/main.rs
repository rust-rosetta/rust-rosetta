#[macro_use]
extern crate structopt;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// The file that lines should be removed from
    #[structopt(parse(from_os_str))]
    file: PathBuf,

    /// The first line number that should be removed (starting at 1)
    start: usize,

    /// The number of lines that should be removed
    count: usize,
}

fn main() {
    let opt = Opt::from_args();

    let file = BufReader::new(File::open(opt.file).unwrap());

    for (i, line) in file.lines().enumerate() {
        let cur = i + 1;

        if cur < opt.start || cur >= (opt.start + opt.count) {
            println!("{}", line.unwrap());
        }
    }
}
