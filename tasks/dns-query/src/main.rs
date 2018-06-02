use std::io;
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};

fn get_ips(host: &str) -> io::Result<impl Iterator<Item = IpAddr>> {
    let hosts = host.to_socket_addrs().unwrap();
    let ips = hosts.map(|h| match h {
        SocketAddr::V4(s_v4) => IpAddr::V4(*s_v4.ip()),
        SocketAddr::V6(s_v6) => IpAddr::V6(*s_v6.ip()),
    });
    Ok(ips)
}

fn main() -> io::Result<()> {
    for ip in get_ips("www.kame.net:80")? {
        match ip {
            IpAddr::V4(ip) => println!("ip v4: {}", ip),
            IpAddr::V6(ip) => println!("ip v6: {}", ip),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ipv4() {
        let host = "203.178.141.194:80";
        if let SocketAddr::V4(addr) = host.to_socket_addrs().unwrap().next().unwrap() {
            let ip = IpAddr::V4(*addr.ip());
            assert!(get_ips("www.kame.net:80").unwrap().any(|x| x == ip));
        } else {
            panic!();
        }
    }

    #[test]
    #[ignore]
    fn ipv6() {
        let host = "2001:200:dff:fff1:216:3eff:feb1:44d7:80";
        if let SocketAddr::V6(addr) = host.to_socket_addrs().unwrap().next().unwrap() {
            let ip = IpAddr::V6(*addr.ip());
            assert!(get_ips("www.kame.net:80").unwrap().any(|x| x == ip));
        } else {
            panic!();
        }
    }
}
