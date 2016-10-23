extern crate reqwest;

use std::io::Read;

fn main() {
    let mut response = match reqwest::get("https://sourceforge.net") {
        Ok(response) => response,
        Err(e) => {
            panic!("error encountered while making request: {:?}", e);
        }
    };
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();

    println!("{}", &body);
}
