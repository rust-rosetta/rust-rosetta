// http://rosettacode.org/wiki/Include_a_file

// The compiler will include either a `test.rs` or `test/mod.rs` (if the first one doesn't exist)
// file.
mod hello_world;

// Additionally, third-party libraries (called `crates` in Rust) can be declared thusly:
extern crate url;

use url::Url;

fn main() {
    // Use a struct included from an external crate.
    println!("{:?}", Url::parse("http://rosettacode.org").unwrap());

    // Though uncommon, it is also possible to include source directly from files with the
    // `include!` macro.
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/include.rs"));
}
