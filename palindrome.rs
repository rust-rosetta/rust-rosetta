// This was written for Rust 0.9

// Simple character by character palindrome detector
fn palindrome(string: &str) -> bool {
    let forward = string.chars();
    let reverse = string.chars_rev();
    let mut both_directions = forward.zip(reverse);
    
    both_directions.count(|(a,b)|{a != b}) == 0
}

// Demonstration code
fn main() {
    let test_strings = ["nope", "eevee", "lalala", "lalalal", "オオオオ"];
    for string in test_strings.iter() {
        println!("{:s} {:b}", *string, palindrome(*string));
    }
}
