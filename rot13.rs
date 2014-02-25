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
	std::str::from_utf8_owned(string.as_bytes().map(|c| rot13u8(*c))).unwrap()
}

fn main () {
	let a =  rot13(~"abc");
	assert_eq!(a, ~"nop");
}