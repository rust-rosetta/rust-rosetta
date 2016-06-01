// http://rosettacode.org/wiki/Walk_a_directory/Non-recursively
extern crate docopt;
extern crate regex;
extern crate rustc_serialize;

use docopt::Docopt;
use regex::Regex;

const USAGE: &'static str = r"
Usage: walk_a_directory_non_recursively <pattern>

Walks the directory tree starting with the current working directory and
print filenames matching <pattern>.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_pattern: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let re = Regex::new(&args.arg_pattern).unwrap();
    let paths = std::fs::read_dir(".").unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let path = path.to_str().unwrap();

        if re.is_match(path) {
            println!("{}", path);
        }
    }
}
