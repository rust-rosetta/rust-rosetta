// Implements http://rosettacode.org/wiki/Check_that_file_exists
#![allow(unstable)]
use std::io::fs::PathExtensions;

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
