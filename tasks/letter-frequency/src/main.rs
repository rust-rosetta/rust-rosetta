#![feature(io)]

use std::io::{BufReader, Read};
use std::collections::HashMap;
use std::fs::File;
use std::collections::hash_map::Entry::{Occupied, Vacant};

fn count_chars<T>(chars: T) -> HashMap<char, usize>
    where T: Iterator<Item = char>
{
    let mut map: HashMap<char, usize> = HashMap::new();
    for letter in chars {
        match map.entry(letter) {
            Vacant(entry) => {
                entry.insert(1);
            }
            Occupied(mut entry) => {
                *entry.get_mut() += 1;
            }
        };
    }
    map
}

fn main() {
    let file = File::open("resources/unixdict.txt").unwrap();
    let reader = BufReader::new(file);

    println!("{:?}", count_chars(reader.chars().map(|c| c.unwrap())));
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
    assert!(map[&'a'] == 4);
    assert!(map[&'b'] == 4);
    assert!(map[&'c'] == 1);
}
