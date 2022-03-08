use std::collections::HashMap;

fn main() {
    let mut original = HashMap::new();
    original.insert("name", "Rocket Skates");
    original.insert("price", "12.75");
    original.insert("color", "yellow");

    let mut update = HashMap::new();
    update.insert("price", "15.25");
    update.insert("color", "red");
    update.insert("year", "1974");

    original.extend(&update);

    println!("{:#?}", original)
}
