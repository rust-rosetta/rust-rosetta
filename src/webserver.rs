// Implements http://rosettacode.org/wiki/Hello_world/Web_server
#![allow(unstable)]

use std::io::net::tcp::{TcpAcceptor, TcpListener, TcpStream};
use std::io::{Acceptor, Listener, IoResult};
use std::thread::Thread;

#[cfg(not(test))] use std::os;

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
    stream.close_write()
}

pub fn handle_server(ip: &str, port: u16) -> IoResult<TcpAcceptor> {
    let listener = try!(TcpListener::bind((ip, port)));
    let mut acceptor = listener.listen();
    println!("Listening for connections on port {}", port);

    let handle = acceptor.clone();
    Thread::spawn(move || -> () {
        for stream in acceptor.incoming() {
            match stream {
                Ok(s) => {
                    Thread::spawn(move || {
                        match handle_client(s) {
                            Ok(_) => println!("Response sent!"),
                            Err(e) => println!("Failed sending response: {}!", e),
                        }
                    });
                },
                Err(e) => {
                    println!("No longer accepting new requests: {}", e);
                    break
                }
            }
        }
    });

    handle
}

#[cfg(not(test))]
fn main() {
    let args = os::args();

    let host = "127.0.0.1";
    let port = if args.len() == 2 {
        args[1].parse::<u16>().expect(&*format!("Usage: {} <port>", args[0]))
    } else {
        80
    };

    handle_server(host, port).unwrap();
}
