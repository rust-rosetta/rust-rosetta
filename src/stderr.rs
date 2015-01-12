// Implements http://rosettacode.org/wiki/Hello_world/Standard_error
#![allow(unstable)]
use std::io;

fn main() {
    let mut stderr = io::stderr();
    let _ = stderr.write(b"Goodbye, World!\n");
}
