// The compiler will import either `hello_world.rs` or `hello_world/mod.rs` (if the first one
// doesn't exist).
mod hello_world;

// You can import public functions, structs, etc. from a module with the `use` statement.
use hello_world::hello_world;

// Additionally, third-party libraries (called `crates` in Rust) can be declared thusly:
extern crate url;

use url::Url;

fn main() {
    // Call an imported function.
    hello_world();

    // Use a struct included from an external crate.
    println!("{:?}", Url::parse("http://rosettacode.org").unwrap());

    // Though uncommon, it is also possible to include source directly from files with the
    // `include!` macro.
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/include.rs"));
}
