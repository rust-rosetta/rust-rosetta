extern crate hyper;

use std::io::prelude::*;

use hyper::Client;

fn main() {
    let client = Client::new();
    let response = {
        let mut response = client.get("http://rosettacode.org").send().unwrap();
        let mut response_text = String::new();
        response.read_to_string(&mut response_text).unwrap();
        response_text
    };

    println!("{}", response);
}
