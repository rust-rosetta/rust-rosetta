// Implements http://rosettacode.org/wiki/Hello_world/Web_server

use std::io::net::tcp::{TcpListener, TcpStream};
use std::io::net::ip::{Ipv4Addr, SocketAddr};
use std::io::{Acceptor, Listener};

fn handle_client(mut stream: TcpStream) {
    let response = bytes!(
"HTTP/1.1 200 OK
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
</html>");
    match stream.write(response) {
        Ok(_) => println!("Response sent!"),
        Err(e) => println!("Failed sending response: {}!", e),
    }
}

fn main() {
    let addr = SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 80 };
    let listener = TcpListener::bind(addr).unwrap();

    let mut acceptor = listener.listen();
    println!("Listening for connections on port {}", addr.port);

    for stream in acceptor.incoming() {
        spawn(proc() {
            handle_client(stream.unwrap());
        });
    }
}
