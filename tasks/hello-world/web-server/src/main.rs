use std::env;
use std::net::{TcpStream, TcpListener, Shutdown};
use std::io::{Write, Result};

fn handle_client(mut stream: TcpStream) -> Result<()> {
    let response = b"HTTP/1.1 200 OK
Content-Type: text/html;
charset=UTF-8

<!doctype html>
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

    try!(stream.write_all(response));
    stream.shutdown(Shutdown::Write)
}

fn handle_server(ip: &str, port: u16) -> Result<TcpListener> {
    use std::thread::spawn;
    let listener = try!(TcpListener::bind((ip, port)));
    println!("Listening for connections on port {}", port);

    let handle = try!(listener.try_clone());
    for stream in handle.incoming() {
        match stream {
            Ok(s) => {
                spawn(move || {
                    match handle_client(s) {
                        Ok(_) => println!("Response sent!"),
                        Err(e) => println!("Failed sending response: {}!", e),
                    }
                });
            }
            Err(e) => {
                println!("No longer accepting new requests: {}", e);
                break;
            }
        }
    }
    drop(listener);
    Ok(handle)
}

fn main() {
    let mut args = env::args();
    let app_name = args.next()
        .unwrap()
        .to_owned();
    let host = "127.0.0.1";
    let port = if let Some(os_port) = args.next() {
        let s_port = os_port.to_owned();
        s_port.parse::<u16>().expect(&*format!("Usage: {:?} <port>", app_name))
    } else {
        8080
    };

    handle_server(host, port).unwrap();
}
