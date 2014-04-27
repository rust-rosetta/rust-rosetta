// Implements http://rosettacode.org/wiki/Palindrome_detection

// Simple character by character palindrome detector
fn palindrome(string: &str) -> bool {
    let forward = string.chars().take(string.len() / 2);
    let reverse = string.chars_rev().take(string.len() / 2);
    let mut both_directions = forward.zip(reverse);
    
    both_directions.all(|(a, b)| {a == b})
}

// Demonstration code
fn main() {
    let test_strings = ["nope", "eevee", "lalala", "lalalal", "オオオオ"];
    for string in test_strings.iter() {
        println!("{:s} {:b}", *string, palindrome(*string));
    }
}
