// Implements http://rosettacode.org/wiki/File_size
#![allow(unstable)]
use std::io::fs::PathExtensions;

fn main() {
    let path_wd = Path::new("input.txt");
    println!("{}", path_wd.stat().unwrap().size);

    let path_root = Path::new("/input.txt");
    println!("{}", path_root.stat().unwrap().size);
}
