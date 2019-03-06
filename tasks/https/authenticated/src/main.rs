extern crate reqwest;

use std::io::Read;

use reqwest::header::CONNECTION;
use reqwest::Client;

fn main() {
    let client = Client::new();

    // reqwest uses strongly-typed structs for creating headers
    let mut res = client
        .get("https://www.example.com")
        .basic_auth("user", Some("password"))
        .header(CONNECTION, "close")
        .send()
        .unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("{}", &body);
}
