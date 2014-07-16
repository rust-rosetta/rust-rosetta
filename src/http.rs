// http://rosettacode.org/wiki/HTTP
use std::io::net::tcp::TcpStream;
use std::io::IoResult;

fn get_index(target: &str) -> IoResult<String> {
    // Create a socket. Mutable so we can write to it.
    let mut socket = TcpStream::connect(target, 80);
    // Write to the socket as bytes.
    // try! and write! are useful macros when working with writers.
    try!(write!(socket, "GET / HTTP/1.1\nHost: {}\n\n", target));
    // Read any response.
    socket.read_to_string()
}

#[cfg(not(test))]
fn main() {
    let target = std::os::args().pop().unwrap();
    println!("Making the request... This might take a minute.");
    match get_index(target.as_slice()) {
        Ok(out) => println!("{}", out),
        Err(e) => println!("Error: {}", e)
    }
}

#[test]
fn test_request() {
    let target = "rust-lang.org";
    match get_index(target) {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e)
    }
}
