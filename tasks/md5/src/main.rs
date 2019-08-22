use md5::{Digest, Md5};
fn main() {
    // digest is a convenience function
    // when you have the full value to hash
    let hash = Md5::digest(b"The quick brown fox jumped over the lazy dog's back");
    // print the hashed value
    // with the handy formatting traits:
    // https://doc.rust-lang.org/std/fmt/index.html#formatting-traits
    println!("{:x}", hash);
}
