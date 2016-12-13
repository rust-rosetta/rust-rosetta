use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// The actual echo server
fn echo_server(host: &'static str, port: u16) -> io::Result<()> {
    // Create a new TCP listener at host:port.
    let listener = try!(TcpListener::bind((host, port)));
    println!("Starting echo server on {:?}", listener.local_addr());

    // Process each new connection to the server
    for stream in listener.incoming() {
        match stream {
            Err(e) => println!("Connection failed: {}", e),
            Ok(stream) => {
                let addr = try!(stream.peer_addr());
                println!("New connection: {}", addr);
                // Launch a new thread to deal with the connection.
                thread::spawn(move || {
                    if let Err(e) = echo_session(stream) {
                        println!("I/O error: {} -- {}", addr, e);
                    }
                    println!("Closing connection: {}", addr);
                });
            }
        }
    }
    Ok(())
    // Server closes automatically at end of block
}

// Each connection gets its own session.
fn echo_session(stream: TcpStream) -> io::Result<()> {
    let addr = try!(stream.peer_addr());
    let mut writer = stream.try_clone().unwrap();
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        let line = try!(line);
        println!("Received line from {}: {}", addr, line);
        try!(writer.write_all(line.as_bytes()));
        println!("Wrote line to {}: {}", addr, line);
    }
    Ok(())
}

const HOST: &'static str = "127.0.0.1";
const PORT: u16 = 12321;

pub fn main() {
    echo_server(HOST, PORT).unwrap();
}
