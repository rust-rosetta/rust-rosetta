use std::io;
use std::net::ToSocketAddrs;

fn main() -> io::Result<()> {
    let host = "www.kame.net:80";

    for ip in host.to_socket_addrs()?.map(|addr| addr.ip()) {
        println!("{}", ip);
    }

    Ok(())
}
