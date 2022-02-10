use std::io;

fn main() {
    io::copy(&mut io::stdin().lock(), &mut io::stdout().lock()).expect("failed to copy");
}
