// Implements http://rosettacode.org/wiki/Echo_server
#![feature(io)]
#![feature(old_io)]
#![feature(std_misc)]
use std::old_io::{Acceptor, BufferedReader, IoError, IoResult, Listener, TimedOut};
use std::old_io::net::tcp::{TcpListener, TcpStream};
use std::time::Duration;
use std::thread::spawn;

// The actual echo server
fn echo_server(host: &'static str, port: u16, timeout: Option<Duration>) -> IoResult<()> {
    // Create a new TCP listener at host:port.
    let mut listener = try!(TcpListener::bind((host, port)));
    println!("Starting echo server on {:?}", listener.socket_name());

    let mut acceptor = try!(listener.listen());
    println!("Echo server started");

    // The server will cease to accept new connectinos after `timeout`.
    acceptor.set_timeout(timeout.map( |d| d.num_milliseconds() as u64));

    // Process each new connection to the server
    for stream in acceptor.incoming() {
        match stream {
            Err(e @ IoError { kind: TimedOut, .. } ) => {
                println!("No longer accepting new requests: {}", e);
                break
            }
            Err(e) => println!("Connection failed: {}", e),
            Ok(mut stream) => {
                let name = try!(stream.peer_name());
                println!("New connection: {}", name);
                // Launch a new thread to deal with the connection.
                spawn(move || -> () {
                    if let Err(e) = echo_session(stream.clone()) {
                        println!("I/O error: {} -- {}", name, e);
                    }
                    println!("Closing connection: {}", name);;
                    drop(stream);
                });
            }
        }
    }
    Ok(())
    // Server closes automatically at end of block
}

// Each connection gets its own session.
fn echo_session(mut stream: TcpStream) -> IoResult<()> {
    let name = try!(stream.peer_name());
    let ref mut writer = stream.clone();
    let mut reader = BufferedReader::new(stream);
    for line in reader.lines() {
        let l = try!(line);
        print!("Received line from {}: {}", name, l);
        try!(writer.write_str(&l[..]));
        print!("Wrote line to {}: {}", name, l);
    }
    Ok(())
}

const HOST: &'static str = "127.0.0.1";
const PORT: u16 = 12321;

pub fn run_server(duration: Option<Duration>) -> IoResult<()> {
    echo_server(HOST, PORT, duration)
}

#[cfg(not(test))]
pub fn main() {
    run_server(Some(Duration::minutes(1))).unwrap();
}
