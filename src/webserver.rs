// Implements http://rosettacode.org/wiki/Hello_world/Web_server
// not_tested

use std::io::net::tcp::{TcpAcceptor, TcpListener, TcpStream};
use std::io::{Acceptor, Listener, IoResult};

fn handle_client(mut stream: TcpStream) -> IoResult<()> {
    let response =
b"HTTP/1.1 200 OK
Content-Type: text/html;
charset=UTF-8

<doctype !html>
<html>
    <head>
        <title>Bye-bye baby bye-bye</title>
        <style>
            body { background-color: #111 }
            h1 { font-size:4cm; text-align: center; color: black; text-shadow: 0 0 2mm red}
        </style>
    </head>
    <body>
        <h1>Goodbye, world!</h1>
    </body>
</html>";

    try!(stream.write(response));
    let _ = stream.close_write();
    Ok(())
}

pub fn handle_server(ip: &'static str, port: u16) -> IoResult<TcpAcceptor> {
    let listener = try!(TcpListener::bind(ip, port));

    let mut acceptor = listener.listen();
    println!("Listening for connections on port {}", port);

    let acceptor_ = acceptor.clone();
    spawn(proc() {
        for stream in acceptor.incoming() {
            match stream {
                Ok(s) => spawn(proc() {
                    match handle_client(s) {
                        Ok(_) => println!("Response sent!"),
                        Err(e) => println!("Failed sending response: {}!", e),
                    }
                }),
                Err(e) => {
                    println!("No longer accepting new requests: {}", e);
                    break
                }
            }
        }
        // close the socket server
        drop(acceptor);
    });
    acceptor_
}

#[cfg(not(test))]
fn main() {
    static HOST: &'static str = "127.0.0.1";
    static PORT: u16 = 80;
    let acceptor = handle_server(HOST, PORT).unwrap();
    drop(acceptor);
}
