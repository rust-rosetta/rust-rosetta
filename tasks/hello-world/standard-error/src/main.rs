use std::io::{self, Write};

fn main() {
    let mut stderr = io::stderr();
    let _ = stderr.write_all(b"Goodbye, World!\n");
}
