// http://rosettacode.org/wiki/HTTP
mod hello_world;

use std::net::TcpStream;
use std::io::{Read, Result, Write};

fn get_index(target: &str, port: u16) -> Result<String> {
    // Create a socket. Mutable so we can write to it.
    let mut socket = try!(TcpStream::connect((target, port)));
    // Write to the socket as bytes.
    // try! and write! are useful macros when working with writers.
    // We send the `Connection: close` header so the server closes the connection
    // after sending its response. This allows us to use `read_to_string()` which
    // reads until EOF. Alternatively, we could use HTTP/1.0. In the future, this
    // will be handled by a HTTP library.
    try!(write!(&mut socket,
                "GET / HTTP/1.1\nHost: {}\nConnection: close\n\n",
                target));
    // Read any response.
    let mut resp = String::new();
    let _ = try!(socket.read_to_string(&mut resp));
    Ok(resp)
}

fn main() {
    use std::borrow::ToOwned;
    const PORT: u16 = 80;

    let target = std::env::args()
        .next()
        .unwrap()
        .to_owned();
    println!("Making the request... This might take a minute.");
    match get_index(&target[..], PORT) {
        Ok(out) => println!("{}", out),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use hello_world::web_server;
    use super::get_index;

    /// FIXME: http/webrequest should eventually be moved to hyper
    #[test]
    #[ignore]
    fn test_request() {
        const HOST: &'static str = "127.0.0.1";
        const PORT: u16 = 12321;

        let (port, _acceptor) = (PORT..::std::u16::MAX)
            .map(|port| (port, web_server::handle_server(HOST, port)))
            .find(|&(_, ref acceptor)| acceptor.is_ok())
            .unwrap();

        let res = get_index(HOST, port);
        res.unwrap();
    }
}
