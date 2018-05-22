#![feature(lookup_host)]

use std::io;
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(PartialEq)]
enum Ip {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

fn get_ips(host: &str) -> io::Result<Vec<Ip>> {
    use std::net::{self, SocketAddr};

    let hosts = net::lookup_host(host)?;
    let ips: Vec<_> = hosts
        .filter_map(|h| match h {
            SocketAddr::V4(s_v4) => Some(Ip::V4(*s_v4.ip())),
            SocketAddr::V6(s_v6) => Some(Ip::V6(*s_v6.ip())),
        })
        .collect();
    Ok(ips)
}

fn main() {
    for ip in &(get_ips("www.kame.net").unwrap()) {
        match *ip {
            Ip::V4(ip) => println!("ip v4: {}", ip),
            Ip::V6(ip) => println!("ip v6: {}", ip),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{get_ips, Ip};
    use std::net::{Ipv4Addr, Ipv6Addr};
    use std::str::FromStr;

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
