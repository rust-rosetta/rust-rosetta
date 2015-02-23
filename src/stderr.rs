// Implements http://rosettacode.org/wiki/Hello_world/Standard_error
#![feature(old_io)]

use std::old_io;

fn main() {
    let mut stderr = old_io::stderr();
    let _ = stderr.write_all(b"Goodbye, World!\n");
}
