fn main() {
    let string = "Hello World";
    assert_eq!(Some("ello World"), string.get(1..));
    assert_eq!(Some("Hello Wor"), string.get(..(string.len() - 2)));
    assert_eq!(Some("lo Wo"), string.get(3..(string.len() - 3)));
    assert_eq!(None, string.get(42..)); // out of bounds
}
