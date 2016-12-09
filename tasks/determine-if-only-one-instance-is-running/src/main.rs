use std::net::TcpListener;

fn create_app_lock(port: u16) -> TcpListener {
    match TcpListener::bind(("0.0.0.0", port)) {
        Ok(socket) => socket,
        Err(_) => {
            panic!("Couldn't lock port {}: another instance already running?",
                   port);
        }
    }
}

fn remove_app_lock(socket: TcpListener) {
    drop(socket);
}

fn main() {
    let lock_socket = create_app_lock(12345);
    // ...
    // your code here
    // ...
    remove_app_lock(lock_socket);
}
