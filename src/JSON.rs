extern crate serialize;
use serialize::json;
#[deriving(Decodable, Encodable)]
struct Penguin {
    name : String,
    born : i16
}
fn main() {
    let pengu = Penguin { name : "pengu".to_string(), born : 1999 };
    println!("{}", json::encode(&pengu));
    let pingu : Penguin = json::decode(r##"{"name":"pingu","born":2001}"##).unwrap();
    assert_eq!(pingu.name.as_slice(), "pingu");
    assert_eq!(pingu.born, 2001);
}
