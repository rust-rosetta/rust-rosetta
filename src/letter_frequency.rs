// Implements http://rosettacode.org/wiki/Letter_frequency

#[cfg(not(test))]
use std::io::fs::File;
#[cfg(not(test))]
use std::io::BufferedReader;
use std::collections::HashMap;
use std::collections::hashmap::{Occupied, Vacant};

fn count_chars<T: Iterator<char>>(mut chars: T) -> HashMap<char, uint> {
    let mut map: HashMap<char, uint> = HashMap::new();
    for letter in chars {
        match map.entry(letter) {
            Vacant(entry) => entry.set(1u),
            Occupied(mut entry) => {
                *entry.get_mut() += 1;
                entry.into_mut()
            }
        };
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
    assert!(map['a'] == 4);
    assert!(map['b'] == 4);
    assert!(map['c'] == 1);
}
