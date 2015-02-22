// Implements http://rosettacode.org/wiki/Rename_a_file
#![feature(old_io)]
#![feature(old_path)]

use std::old_io::fs;

fn main() {
    fs::rename(&Path::new("input.txt"), &Path::new("output.txt")).unwrap();
    fs::rename(&Path::new("docs"), &Path::new("mydocs")).unwrap();
    fs::rename(&Path::new("/input.txt"), &Path::new("/output.txt")).unwrap();
    fs::rename(&Path::new("/docs"), &Path::new("/mydocs")).unwrap();
}
