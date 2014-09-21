// Implements http://rosettacode.org/wiki/Letter_frequency

#[cfg(not(test))]
use std::io::fs::File;
#[cfg(not(test))]
use std::io::BufferedReader;
use std::collections::HashMap;

fn count_chars<T: Iterator<char>>(mut chars: T) -> HashMap<char, uint> {
    let mut map: HashMap<char, uint> = HashMap::new();
    for letter in chars {
        map.insert_or_update_with(letter, 1, |_, count| *count += 1);
    }
    map
}

#[cfg(not(test))]
fn main() {
    let file = File::open(&Path::new("resources/unixdict.txt"));
    let mut reader = BufferedReader::new(file);

    println!("{}", count_chars(reader.chars().map(|c| c.unwrap())));
}

#[test]
fn test_empty() {
    let map = count_chars("".chars());
    assert!(map.len() == 0);
}

#[test]
fn test_basic() {
    let map = count_chars("aaaabbbbc".chars());

    assert!(map.len() == 3);
    assert!(*map.get(&'a') == 4);
    assert!(*map.get(&'b') == 4);
    assert!(*map.get(&'c') == 1);
}
