fn rot13 (string: ~str) -> ~str {
	fn rot13u8 (c: u8) -> u8 {
		match c {
			97..109 => c+13,
			65..77 => c+13,
			110..122 => c-13,
			78..90 => c-13,
			_ => c
		}
	}
        let translated = string.as_bytes().iter().map(|c| rot13u8(*c)).collect();
	std::str::from_utf8_owned(translated).unwrap()
}

fn main () {
	let a =  rot13(~"abc");
	assert_eq!(a, ~"nop");
}
