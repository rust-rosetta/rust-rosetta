// Implements http://rosettacode.org/wiki/JSON

extern crate serialize;

use serialize::{json, Encodable, Decodable};

#[deriving(Show, Encodable, Decodable, PartialEq, Eq)]
pub struct Contact {
    name: String,
    city: String
}

#[cfg(not(test))]
fn main() {
    // Encode contact to json
    let c = Contact { name: "John".to_str(), city: "Paris".to_str() };
    let json = encode_contact(&c);
    println!("Encoded: {}", json.as_slice());
    
    // Decode json to contact
    let json_str = "{\"name\":\"Alan\", \"city\":\"Tokyo\"}";
    let contact = decode_contact(json_str);
    println!("Decoded: {}", contact);
}

fn encode_contact(data: &Contact) -> String {
    json::Encoder::str_encode(&data)
}

fn decode_contact(json_str: &str) -> Contact {
    let json_obj = json::from_str(json_str).unwrap();
    let mut decoder = json::Decoder::new(json_obj);
    Decodable::decode(&mut decoder).unwrap()
}

#[test]
fn test_coherence() {
    let c = Contact { name: "John".to_str(), city: "Paris".to_str() };
    assert!(decode_contact(encode_contact(&c).as_slice()) == c);
}

#[test]
fn test_decode() {
    let json_str = "{\"name\":\"Alan\", \"city\":\"Tokyo\"}";
    let contact = decode_contact(json_str);
    assert!(contact == Contact { name: "Alan".to_str(), city: "Tokyo".to_str() });
}
