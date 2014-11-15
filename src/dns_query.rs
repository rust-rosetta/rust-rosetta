// Implements http://rosettacode.org/wiki/DNS_query 

use std::io::net::addrinfo::get_host_addresses;
use std::io::net::ip::IpAddr;

fn get_ips(host: &str) -> Vec<IpAddr> {
    let mut ips = match get_host_addresses(host) {
        Ok(ips) => ips,
        Err(err) => panic!("{}", err)
    };

    ips.dedup();

    ips
}

#[cfg(not(test))]
fn main() {
    for ip in get_ips("www.kame.net").iter() {
        println!("{}", ip);
    }
}

#[test]
fn ipv4() {
    let ip: IpAddr = from_str("203.178.141.194").unwrap();
    assert!(get_ips("www.kame.net").contains(&ip));
}

#[test]
fn ipv6() {
    let ip: IpAddr = from_str("2001:200:dff:fff1:216:3eff:feb1:44d7").unwrap();
    assert!(get_ips("www.kame.net").contains(&ip));
}
