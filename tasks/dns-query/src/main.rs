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

    let hosts = try!(net::lookup_host(host));
    let ips: Vec<_> = hosts.filter_map(|h| {
            match h {
                SocketAddr::V4(s_v4) => Some(Ip::V4(*s_v4.ip())),
                SocketAddr::V6(s_v6) => Some(Ip::V6(*s_v6.ip())),
            }
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
    use super::{Ip, get_ips};
    use std::net::{Ipv4Addr, Ipv6Addr};
    use std::str::FromStr;

    #[test]
    fn ipv4() {
        let ip = Ip::V4(Ipv4Addr::from_str("203.178.141.194").unwrap());
        assert!(get_ips("www.kame.net").unwrap().contains(&ip));
    }

    #[test]
    #[ignore]
    fn ipv6() {
        let ip = Ip::V6(Ipv6Addr::from_str("2001:200:dff:fff1:216:3eff:feb1:44d7").unwrap());
        assert!(get_ips("www.kame.net").unwrap().contains(&ip));
    }
}
