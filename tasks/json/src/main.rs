#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Contact {
    name: String,
    city: String,
}

fn main() {
    // Encode contact to json
    let c = Contact {
        name: "John".to_string(),
        city: "Paris".to_string(),
    };
    let json = serde_json::to_string(&c).unwrap();
    println!("Encoded: {:?}", json);

    // Decode json to contact
    let json_str = r#"{ "name": "Alan", "city": "Tokyo" }"#;
    let contact: Contact = serde_json::from_str(json_str).unwrap();
    println!("Decoded: {:?}", contact);
}

#[test]
fn test_coherence() {
    let c = Contact {
        name: "John".to_string(),
        city: "Paris".to_string(),
    };
    assert_eq!(serde_json::from_str::<Contact>(&serde_json::to_string(&c).unwrap()).unwrap(), c);
}

#[test]
fn test_decode() {
    let json_str = r#"{ "name": "Alan", "city": "Tokyo" }"#;
    let contact: Contact = serde_json::from_str(json_str).unwrap();
    assert_eq!(contact, Contact {
        name: "Alan".to_string(),
        city: "Tokyo".to_string(),
    });
}
