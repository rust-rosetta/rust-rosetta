// Implements http://rosettacode.org/wiki/DNS_query
use std::io;
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(PartialEq)]
enum Ips {
    IpV4(Ipv4Addr),
    IpV6(Ipv6Addr),
}

fn get_ips(host: &str) -> io::Result<Vec<Ips>> {
    use std::net::{self, SocketAddr};
    use Ips::{IpV4, IpV6};

    let hosts = try!(net::lookup_host(host));
    let ips: Vec<_> = hosts.filter_map(|h|
        match h {
            Ok(SocketAddr::V4(s_v4)) => Some(IpV4(s_v4.ip().clone())),
            Ok(SocketAddr::V6(s_v6)) => Some(IpV6(s_v6.ip().clone())),
            _ => None,
        }
    ).collect();
    Ok(ips)
}

#[cfg(not(test))]
fn main() {
    for ip in &(get_ips("www.kame.net").unwrap()) {
        match ip {
            &Ips::IpV4(ip) => println!("ip v4: {}", ip),
            &Ips::IpV6(ip) => println!("ip v6: {}", ip)
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Ips, get_ips};
    use std::net::{Ipv4Addr, Ipv6Addr};
    use std::str::FromStr;

    #[test]
    fn ipv4() {
        let ip = Ips::IpV4(Ipv4Addr::from_str("203.178.141.194").unwrap());
        assert!(get_ips("www.kame.net").unwrap().contains(&ip));
    }

    #[test]
    #[ignore(cfg(target_os = "win32"))]
    fn ipv6() {
        let ip = Ips::IpV6(Ipv6Addr::from_str("2001:200:dff:fff1:216:3eff:feb1:44d7").unwrap());
        assert!(get_ips("www.kame.net").unwrap().contains(&ip));
    }
}
