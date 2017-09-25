extern crate reqwest;

use std::io::Read;

use reqwest::Client;
use reqwest::header::{Authorization, Basic, Connection};

fn main() {
    let client = Client::new();

    // reqwest uses strongly-typed structs for creating headers
    let mut res = client
        .get("https://www.example.com")
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
