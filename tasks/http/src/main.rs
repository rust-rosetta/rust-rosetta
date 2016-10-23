extern crate reqwest;

use std::io::prelude::*;

fn main() {
    let mut response = reqwest::get("http://rosettacode.org").unwrap();
    let mut response_text = String::new();
    response.read_to_string(&mut response_text).unwrap();

    println!("{}", response_text);
}
