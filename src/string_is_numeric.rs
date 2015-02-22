// Implements http://rosettacode.org/wiki/Determine_if_a_string_is_numeric

fn is_numeric (s: &str) -> bool {
	s.parse::<f64>().is_ok()
}

#[cfg(not(test))]
fn main() {
	println!("{}", is_numeric("4.034"));
}

#[test]
fn test_isize() {
	assert!(is_numeric("4"));
	assert!(is_numeric("-4"));
}

#[test]
#[should_fail]
fn test_str() {
	assert!(is_numeric("j"));
}

#[test]
fn test_float() {
	assert!(is_numeric("1.034"));
	assert!(is_numeric("-1.034"));
}