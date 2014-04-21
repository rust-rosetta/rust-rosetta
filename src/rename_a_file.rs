use std::io::fs;

fn main() {
    fs::rename(&Path::new("input.txt"), &Path::new("output.txt")).unwrap();
    fs::rename(&Path::new("docs"), &Path::new("mydocs")).unwrap();
    fs::rename(&Path::new("/input.txt"), &Path::new("/output.txt")).unwrap();
    fs::rename(&Path::new("/docs"), &Path::new("/mydocs")).unwrap();
}
