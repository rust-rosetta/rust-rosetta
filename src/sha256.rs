// http://rosettacode.org/wiki/SHA-256

extern crate crypto;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

fn main() {
    println!("{}", sha_256("Rosetta code"));
}

fn sha_256(input: &str) -> String {
    let mut sh = Sha256::new();
    sh.input_str(input);
    sh.result_str()
}

#[test]
fn test_rosetta() {
    assert_eq!(sha_256("Rosetta code"),
               "764faf5c61ac315f1497f9dfa542713965b785e5cc2f707d6468d7d1124cdfcf".to_string());
}
