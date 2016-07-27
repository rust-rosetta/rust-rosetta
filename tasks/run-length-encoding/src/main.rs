extern crate run_length_encoding;

use run_length_encoding::{INPUT, encode, decode};

fn main() {
    let enc = encode(INPUT);
    println!("encoded {}", enc);

    let dec = decode(&enc[..]);
    println!("decoded {}", dec.unwrap());
}
