// http://rosettacode.org/wiki/HTTPS/Authenticated

extern crate hyper;

use std::io::Read;

use hyper::Client;
use hyper::header::{Authorization, Basic, Connection};

fn main() {
    let client = Client::new();

    // hyper uses strongly-typed structs for creating headers
    let mut res = client.get("https://www.example.com")
        .header(Authorization(Basic {
            username: String::from("user"),
            password: Some(String::from("password")),
        }))
        .header(Connection::close())
        .send()
        .unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("{}", &body);
}
