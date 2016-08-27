use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    // Open a tcp socket connecting to 127.0.0.1:256, no error handling (unwrap)
    let mut my_stream = TcpStream::connect("127.0.0.1:256").unwrap();

    // Write 'hello socket world' to the stream
    write!(my_stream, "hello socket world").unwrap();
} // <- my_stream's drop function gets called, which closes the socket
