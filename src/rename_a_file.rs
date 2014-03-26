use std::io::fs;

fn main() {
    fs::rename(&Path::new("input.txt"), &Path::new("output.txt")).ok();
    fs::rename(&Path::new("docs"), &Path::new("mydocs")).ok();
    fs::rename(&Path::new("/input.txt"), &Path::new("/output.txt")).ok();
    fs::rename(&Path::new("/docs"), &Path::new("/mydocs")).ok();
}
