extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

fn main() {
    let mut sh = Md5::new();
    sh.input_str("The quick brown fox jumped over the lazy dog's back");
    println!("{}", sh.result_str());
}
