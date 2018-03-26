use std::net::TcpListener;

fn create_app_lock(port: u16) -> TcpListener {
    match TcpListener::bind(("0.0.0.0", port)) {
        Ok(socket) => socket,
        Err(e) => {
            panic!(
                "Couldn't lock port {}: another instance already running? ({})",
                port, e
            );
        }
    }
}

fn remove_app_lock(socket: TcpListener) {
    drop(socket);
}

fn main() {
    #[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
    let lock_socket = create_app_lock(12345);
    // ...
    // your code here
    // ...
    remove_app_lock(lock_socket);
}
