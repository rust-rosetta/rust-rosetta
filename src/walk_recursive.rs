// Implements http://rosettacode.org/wiki/Walk_a_directory/Recursively
#![feature(plugin)]
#![plugin(regex_macros)]
#![feature(std_misc)]
extern crate regex;

use regex::Regex;
use std::fs;
use std::path::AsPath;

fn walk<P>(pth: P, regex: &Regex) where P: AsPath {
    let result = match fs::read_dir(pth.as_path()) {
        Ok(result) => result,
        Err(_) => return
    };

    for subpath in result {
        if let Ok(subp) = subpath {
            if let Ok(filename) = subp.path().into_os_string().into_string() {
                if regex.is_match(&filename) {
                    println!("{}", filename);
                }
                walk(filename, regex);
            }
        }
    }
}

fn main() {
    walk(".", &regex!(r".*\.rs"));
}
