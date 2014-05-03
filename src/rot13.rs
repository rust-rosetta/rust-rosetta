// Implements http://rosettacode.org/wiki/Rot-13

fn rot13 (string: &str) -> ~str {
	fn rot13u8 (c: u8) -> u8 {
		match c {
			97..109 => c+13,
			65..77 => c+13,
			110..122 => c-13,
			78..90 => c-13,
			_ => c
		}
	}
    
    let translated = string.as_bytes().iter().map(|&c| rot13u8(c)).collect();
	std::str::from_utf8_owned(translated).unwrap()
}

#[cfg(not(test))]
fn main () {
    let string = "Do you love apples?";
    
    println!("Original: {}", string);
    println!("Encoded: {}", rot13(string));
}

#[test]
fn test_basic() {
    assert!(rot13("abc") == ~"nop");
}

#[test]
fn test_coherence() {
    assert!(range(50000, 50050).all(|x| rot13(rot13(x.to_str())) == x.to_str()));
}