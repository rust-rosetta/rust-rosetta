use std::fs;

fn main() {
    fs::rename("input.txt", "output.txt").unwrap();
    fs::rename("docs", "mydocs").unwrap();
    fs::rename("/input.txt", "/output.txt").unwrap();
    fs::rename("/docs", "/mydocs").unwrap();
}
