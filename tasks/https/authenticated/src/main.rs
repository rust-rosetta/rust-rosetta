extern crate reqwest;

use reqwest::blocking::Client;
use reqwest::header::CONNECTION;

fn main() {
    let client = Client::new();

    // reqwest uses strongly-typed structs for creating headers
    let res = client
        .get("https://www.example.com")
        .basic_auth("user", Some("password"))
        .header(CONNECTION, "close")
        .send()
        .unwrap();

    let body = res.text().unwrap();

    println!("{}", body);
}
