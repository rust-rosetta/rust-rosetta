fn rot13(string: &str) -> String {
    fn rot13u8(c: char) -> char {
        let d = c as u8;
        match c {
            'a'...'m' | 'A'...'M' => (d + 13) as char,
            'n'...'z' | 'N'...'Z' => (d - 13) as char,
            _ => c,
        }
    }

    string.chars().map(rot13u8).collect()
}

fn main() {
    let string = "Do you love apples?";

    println!("Original: {}", string);
    println!("Encoded: {}", rot13(string));
}

#[test]
fn test_basic() {
    assert_eq!(rot13("abc"), "nop");
}

#[test]
fn test_coherence() {
    let coherence_test = (50000i32..50050).map(|x| format!("{}", x)).all(|s| {
        let encoded = rot13(&s[..]);
        let decoded = rot13(&encoded[..]);
        decoded == s
    });

    assert!(coherence_test);
}
