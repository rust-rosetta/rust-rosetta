// Implements http://rosettacode.org/wiki/SHA-256
#![allow(unused_features)]
#![feature(rustc_private)]
#![feature(collections)]

// note that for now the rustc::util::sha2::Sha256 docs state:
// This implementation is not intended for external use or for any use where security is
// important.
extern crate rustc;
use rustc::util::sha2::{Sha256, Digest};

#[cfg(not(test))]
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
    "764faf5c61ac315f1497f9dfa542713965b785e5cc2f707d6468d7d1124cdfcf"
    .to_string());
}
