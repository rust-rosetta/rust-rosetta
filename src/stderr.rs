// http://rosettacode.org/wiki/Hello_world/Standard_error
use std::io::{self, Write};

fn main() {
    let mut stderr = io::stderr();
    let _ = stderr.write_all(b"Goodbye, World!\n");
}
