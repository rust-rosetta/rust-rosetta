// Implements http://rosettacode.org/wiki/Walk_a_directory/Recursively
#![feature(old_path)]
#![feature(old_io)]

#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;

use regex::Regex;
use std::old_io::fs::readdir;

fn walk(path: &Path, regex: &Regex) {
    let result = match readdir(path) {
        Ok(result) => result,
        Err(_) => return
    };

    for subpath in &result {
        match subpath.filename_str() {
            Some(filename) => {
                if regex.is_match(filename) {
                    println!("{}", subpath.display());
                }
            },
            None => {}
        }

        walk(subpath, regex);
    }
}

fn main() {
    walk(&Path::new("."), &regex!(r".*\.rs"));
}
