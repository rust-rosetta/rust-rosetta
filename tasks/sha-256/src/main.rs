// http://rosettacode.org/wiki/SHA-256

extern crate ring;

use ring::digest::{digest, SHA256};

fn main() {
    println!("{}", sha_256("Rosetta code"));
}

fn sha_256(input: &str) -> String {
    let result = digest(&SHA256, input.as_bytes());
    result.as_ref().iter().map(|b| format!("{:x}", b)).collect()
}

#[test]
fn test_rosetta() {
    let expected_hash = "764faf5c61ac315f1497f9dfa542713965b785e5cc2f707d6468d7d1124cdfcf";
    assert_eq!(sha_256("Rosetta code"), expected_hash);
}
