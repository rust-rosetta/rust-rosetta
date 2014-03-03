use std::io::fs;

fn main() {
    fs::rename(&Path::new("input.txt"), &Path::new("output.txt"));
    fs::rename(&Path::new("docs"), &Path::new("mydocs"));
    fs::rename(&Path::new("/input.txt"), &Path::new("/output.txt"));
    fs::rename(&Path::new("/docs"), &Path::new("/mydocs"));
}
