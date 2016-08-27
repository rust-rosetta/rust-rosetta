//! Rust has very powerful string interpolation. [Documentation here][doc]
//!
//! [doc]: https://doc.rust-lang.org/stable/std/fmt/

fn main() {
    println!("Mary had a {} lamb", "little");
    // You can specify order
    println!("{1} had a {0} lamb", "little", "Mary");
    // Or named arguments if you prefer
    println!("{name} had a {adj} lamb", adj = "little", name = "Mary");
}
