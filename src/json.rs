// Implements http://rosettacode.org/wiki/JSON
extern crate "rustc-serialize" as rustc_serialize;
use rustc_serialize::json;

#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq, Eq)]
pub struct Contact {
    name: String,
    city: String
}

#[cfg(not(test))]
fn main() {
    // Encode contact to json
    let c = Contact { name: "John".to_string(), city: "Paris".to_string() };
    let json = json::encode(&c);
    println!("Encoded: {:?}", &(json.unwrap()));

    // Decode json to contact
    let json_str = "{\"name\":\"Alan\", \"city\":\"Tokyo\"}";
    let contact: Contact = json::decode(json_str).unwrap();
    println!("Decoded: {:?}", contact);
}

#[test]
fn test_coherence() {
    let c = Contact { name: "John".to_string(), city: "Paris".to_string() };
    assert!(json::decode::<Contact>(&(json::encode(&c)).unwrap()) == Ok(c));
}

#[test]
fn test_decode() {
    let json_str = "{\"name\":\"Alan\", \"city\":\"Tokyo\"}";
    let contact: Contact = json::decode(json_str).unwrap();
    assert!(contact == Contact { name: "Alan".to_string(), city: "Tokyo".to_string() });
}
