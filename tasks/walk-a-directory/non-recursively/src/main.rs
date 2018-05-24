#[macro_use]
extern crate structopt;
extern crate regex;

use regex::Regex;
use structopt::StructOpt;

/// Walks the directory tree, starting with the current working directory. Print filenames matching
/// the regular expression `PATTERN`.
#[derive(Debug, StructOpt)]
struct Opt {
    /// Regular expression pattern
    #[structopt(name = "PATTERN")]
    pattern: Regex,
}

fn main() {
    let opt = Opt::from_args();
    let paths = std::fs::read_dir(".").unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let path = path.to_str().unwrap();

        if opt.pattern.is_match(path) {
            println!("{}", path);
        }
    }
}
