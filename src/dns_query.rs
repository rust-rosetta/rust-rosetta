// Implements http://rosettacode.org/wiki/DNS_query
#![allow(unused_features)]
#![feature(old_io)]
#![feature(collections)]


use std::old_io::net::addrinfo::get_host_addresses;
use std::old_io::net::ip::IpAddr;

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
    for ip in &get_ips("www.kame.net") {
        println!("{}", ip);
    }
}

#[test]
fn ipv4() {
    let ip = "203.178.141.194".parse::<IpAddr>().unwrap();
    assert!(get_ips("www.kame.net").contains(&ip));
}

#[test]
#[ignore(cfg(target_os = "win32"))]
fn ipv6() {
    let ip = "2001:200:dff:fff1:216:3eff:feb1:44d7".parse::<IpAddr>().unwrap();
    assert!(get_ips("www.kame.net").contains(&ip));
}
