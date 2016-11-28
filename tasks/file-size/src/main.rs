use std::fs;

fn main() {
    if let Ok(attr) = fs::metadata("input.txt") {
        println!("size: {}", attr.len());
    }
    if let Ok(attr_root) = fs::metadata("/input.txt") {
        println!("{}", attr_root.len());
    }
}
