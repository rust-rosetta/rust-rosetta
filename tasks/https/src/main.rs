extern crate reqwest;

fn main() {
    let response = match reqwest::blocking::get("https://sourceforge.net") {
        Ok(response) => response,
        Err(e) => panic!("error encountered while making request: {:?}", e),
    };

    println!("{}", response.text().unwrap());
}
