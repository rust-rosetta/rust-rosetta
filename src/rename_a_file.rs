// Implements http://rosettacode.org/wiki/Rename_a_file

#[cfg(not(test))]
use std::io::fs;

#[cfg(not(test))]
fn main() {
    fs::rename(&Path::new("input.txt"), &Path::new("output.txt")).unwrap();
    fs::rename(&Path::new("docs"), &Path::new("mydocs")).unwrap();
    fs::rename(&Path::new("/input.txt"), &Path::new("/output.txt")).unwrap();
    fs::rename(&Path::new("/docs"), &Path::new("/mydocs")).unwrap();
}
