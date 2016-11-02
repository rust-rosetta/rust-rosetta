/// Returns true if the string is a palindrome
fn palindrome(string: &str) -> bool {
    // The first part of the string
    let forward = string.chars().take(string.len() / 2);

    // The second part of the string in reverse order
    let reverse = string.chars().rev().take(string.len() / 2);

    // We group the two parts of the string in tuples
    let mut both_directions = forward.zip(reverse);

    // The word is a palindrome if each tuple contains two times
    // the same character
    both_directions.all(|(a, b)| a == b)
}

fn main() {
    let test_strings = ["nope", "eevee", "lalala", "rust", "lalalal"];
    for &string in &test_strings {
        println!("{}: {}", string, palindrome(string));
    }
}

#[test]
fn test_palindromes() {
    let palindromes = ["eevee", "lalalal", "オオオオ", "", "anna"];
    let non_palindromes = ["nope", "lalala", "car", "rain", "house", "computer", "rust"];

    assert!(palindromes.iter().all(|&s| palindrome(s)));
    assert!(non_palindromes.iter().all(|&s| !palindrome(s)));
}
