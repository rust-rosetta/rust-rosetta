use std::collections::HashMap;
use std::io::BufReader;
use std::io::prelude::*;
use std::io;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;

type Username = String;

/// Sends a message to all clients except the sending client.
fn broadcast_message(user: &str,
                     clients: &mut HashMap<String, TcpStream>,
                     message: &str)
                     -> io::Result<()> {

    for (client, stream) in clients.iter_mut() {
        if client != user {
            try!(writeln!(stream, "{}", message));
        }
    }

    Ok(())
}

fn chat_loop(listener: TcpListener) -> io::Result<()> {
    let local_clients: Arc<RwLock<HashMap<Username, TcpStream>>> =
        Arc::new(RwLock::new(HashMap::new()));

    println!("Accepting connections on {}",
             try!(listener.local_addr()).port());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let client_clients = local_clients.clone();
                thread::spawn(move || -> io::Result<()> {
                    let mut reader = BufReader::new(try!(stream.try_clone()));
                    let mut writer = stream;

                    let mut name = String::new();
                    loop {
                        try!(write!(writer, "Please enter a username: "));
                        try!(reader.read_line(&mut name));
                        name = name.trim().to_owned();

                        let clients = client_clients.read().unwrap();
                        if !clients.contains_key(&name) {
                            try!(writeln!(writer, "Welcome, {}!", &name));
                            break;
                        }

                        try!(writeln!(writer, "That username is taken."));
                        name.clear();
                    }

                    {
                        let mut clients = client_clients.write().unwrap();
                        clients.insert(name.clone(), writer);
                        try!(broadcast_message(&name,
                                               &mut *clients,
                                               &format!("{} has joined the chat room.", &name)));
                    }

                    for line in reader.lines() {
                        let mut clients = client_clients.write().unwrap();
                        try!(broadcast_message(&name,
                                               &mut *clients,
                                               &format!("{}: {}", &name, try!(line))));
                    }

                    {
                        let mut clients = client_clients.write().unwrap();
                        clients.remove(&name);
                        try!(broadcast_message(&name,
                                               &mut *clients,
                                               &format!("{} has left the chat room.", &name)));
                    }

                    Ok(())
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}

fn main() {
    let listener = TcpListener::bind(("localhost", 7000)).unwrap();
    chat_loop(listener).unwrap();
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, BufWriter};
    use std::io::prelude::*;
    use std::net::{TcpListener, TcpStream, ToSocketAddrs};
    use std::thread;

    fn create_client<A>(addr: A) -> (BufReader<TcpStream>, BufWriter<TcpStream>)
        where A: ToSocketAddrs
    {
        let client = TcpStream::connect(addr).unwrap();
        let reader = BufReader::new(client.try_clone().unwrap());
        let writer = BufWriter::new(client.try_clone().unwrap());

        (reader, writer)
    }

    /// In this test, a single client logs into the chat server. They should obtain a greeting once
    /// they enter their name.
    #[test]
    fn single_client() {
        let listener = TcpListener::bind(("localhost", 7000)).unwrap();
        let _ = thread::spawn(move || {
            super::chat_loop(listener).unwrap();
        });

        let (mut reader, mut writer) = create_client("localhost:7000");

        writeln!(writer, "client").unwrap();
        writer.flush().unwrap();

        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        assert_eq!("Please enter a username: Welcome, client!\n", line);
        line.clear();

        // Send a message.
        writeln!(writer, "Hello, world!").unwrap();
        writer.flush().unwrap();
    }

    /// In this test, there are two clients. One client will log in successfully. Then, a second
    /// client will log in. The first client should see a message notifying them that the second
    /// client has logged in. The first client will then send a chat message, which the second
    /// client should see. Finally, the second client will close their connection, which should
    /// generate a message for the first client indicating that the second client has logged out.
    #[test]
    fn multi_client() {
        let listener = TcpListener::bind(("localhost", 8000)).unwrap();
        let _ = thread::spawn(move || {
            super::chat_loop(listener).unwrap();
        });

        let (mut reader1, mut writer1) = create_client("localhost:8000");

        writeln!(writer1, "client_1").unwrap();
        writer1.flush().unwrap();

        let mut line = String::new();
        reader1.read_line(&mut line).unwrap();
        assert_eq!("Please enter a username: Welcome, client_1!\n", line);
        line.clear();

        let (mut reader2, mut writer2) = create_client("localhost:8000");
        writeln!(writer2, "client_2").unwrap();
        writer2.flush().unwrap();

        reader2.read_line(&mut line).unwrap();
        assert_eq!("Please enter a username: Welcome, client_2!\n", line);
        line.clear();

        reader1.read_line(&mut line).unwrap();
        assert_eq!("client_2 has joined the chat room.\n", line);
        line.clear();

        writeln!(writer1, "Hello, world!").unwrap();
        writer1.flush().unwrap();

        reader2.read_line(&mut line).unwrap();
        assert_eq!("client_1: Hello, world!\n", line);
        line.clear();

        drop(writer2);
        drop(reader2);

        reader1.read_line(&mut line).unwrap();
        assert_eq!("client_2 has left the chat room.\n", line);
    }

    /// Ensures that a user cannot have the username of a user already in the chat room.
    #[test]
    fn existing_name() {
        let listener = TcpListener::bind(("localhost", 9000)).unwrap();
        let _ = thread::spawn(move || {
            super::chat_loop(listener).unwrap();
        });

        let (mut reader1, mut writer1) = create_client("localhost:9000");

        writeln!(writer1, "client").unwrap();
        writer1.flush().unwrap();

        let mut line = String::new();
        reader1.read_line(&mut line).unwrap();
        assert_eq!("Please enter a username: Welcome, client!\n", line);
        line.clear();

        let (mut reader2, mut writer2) = create_client("localhost:9000");

        writeln!(writer2, "client").unwrap();
        writer2.flush().unwrap();

        reader2.read_line(&mut line).unwrap();
        assert_eq!("Please enter a username: That username is taken.\n", line);
        line.clear();
    }
}
