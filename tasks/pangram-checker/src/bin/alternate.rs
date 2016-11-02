use std::collections::HashSet;

/// Returns true if the sentence uses all 26 letters in the English
/// alphabet at least once.
fn is_pangram(sentence: &str) -> bool {
    sentence.chars()
        .map(|c| c.to_lowercase().next().unwrap())
        .filter(|&c| c >= 'a' && c <= 'z')
        .collect::<HashSet<char>>()
        .len() == 26
}

fn main() {
    let test_sentences = ["The quick brown fox jumps over the lazy dog.",
                          "The quick brown frog jumps over the lazy dog."];
    for &sentence in &test_sentences {
        println!("\"{}\" {} a pangram",
                 sentence,
                 if is_pangram(sentence) { "is" } else { "is not" });
    }
}

#[test]
fn test_empty() {
    assert_eq!(is_pangram(""), false);
}

#[test]
fn test_one_letter() {
    assert_eq!(is_pangram("A"), false);
}

#[test]
fn test_pangram() {
    assert_eq!(is_pangram("The quick brown fox jumps over the lazy dog"),
               true);
}

#[test]
fn test_non_pangram() {
    assert_eq!(is_pangram("The quick brown fox jumps over the lurking dog"),
               false);
}

#[test]
fn test_pangram_unicode() {
    assert_eq!(is_pangram("The quick brown fox (狐狸) jumps over the lazy dog"),
               true);
}

#[test]
fn test_non_pangram_unicode() {
    assert_eq!(is_pangram("The quick brown 狐狸 jumps over the lazy dog"),
               false);
}
