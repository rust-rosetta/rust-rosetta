// Implements http://rosettacode.org/wiki/Rot-13

fn rot13 (string: &str) -> ~str {
	fn rot13u8 (c: char) -> char {
            let d = c as u8;
            match c {
		'a' .. 'm' => (d + 13) as char,
		'n' .. 'z' => (d - 13) as char,
		'A' .. 'M' => (d + 13) as char,
		'N' .. 'Z' => (d - 13) as char,
		_ => c
	    }
	}

    let translated: Vec<char> = string.chars().map(|c| rot13u8(c)).collect();
    std::str::from_chars(translated.as_slice())
}

#[cfg(not(test))]
fn main () {
    let string = "Do you love apples?";

    println!("Original: {}", string);
    println!("Encoded: {}", rot13(string));
}

#[test]
fn test_basic() {
    assert!(rot13("abc") == "nop".to_owned());
}

#[test]
fn test_coherence() {
    assert!(range(50000, 50050).all(|x|
                                    rot13(rot13(x.to_str())) == x.to_str()));
}
