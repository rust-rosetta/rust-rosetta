// Implements http://rosettacode.org/wiki/Check_that_file_exists
#![feature(path_ext)]
use std::path::Path;
use std::fs::PathExt;

fn main() {
    let paths = ["input.txt", "docs"];
    for path in paths.iter().map(|&x| Path::new(x)) {
        let msg = match path.exists() {
            true => "exists",
            false => "does not exist"
        };

        println!("{} {}.", path.display(), msg);
    }
}
