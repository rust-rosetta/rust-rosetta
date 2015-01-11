// Implements http://rosettacode.org/wiki/Walk_a_directory/Recursively
#![allow(unstable)]

#![feature(plugin)]
#[plugin]
extern crate regex_macros;
extern crate regex;

use regex::Regex;
use std::io::fs::readdir;

fn walk(path: &Path, regex: &Regex) {
    let result = match readdir(path) {
        Ok(result) => result,
        Err(_) => return
    };

    for subpath in result.iter() {
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
