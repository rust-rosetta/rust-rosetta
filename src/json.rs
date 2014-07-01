// Implements http://rosettacode.org/wiki/JSON

extern crate serialize;

use serialize::json;

#[deriving(Show, Encodable, Decodable, PartialEq, Eq)]
pub struct Contact {
    name: String,
    city: String
}

#[cfg(not(test))]
fn main() {
    // Encode contact to json
    let c = Contact { name: "John".to_str(), city: "Paris".to_str() };
    let json = json::encode(&c);
    println!("Encoded: {}", json.as_slice());

    // Decode json to contact
    let json_str = "{\"name\":\"Alan\", \"city\":\"Tokyo\"}";
    let contact: Contact = json::decode(json_str).unwrap();
    println!("Decoded: {}", contact);
}

#[test]
fn test_coherence() {
    let c = Contact { name: "John".to_str(), city: "Paris".to_str() };
    assert!(json::decode::<Contact>(json::encode(&c).as_slice()).unwrap() == c);
}

#[test]
fn test_decode() {
    let json_str = "{\"name\":\"Alan\", \"city\":\"Tokyo\"}";
    let contact: Contact = json::decode(json_str).unwrap();
    assert!(contact == Contact { name: "Alan".to_str(), city: "Tokyo".to_str() });
}
