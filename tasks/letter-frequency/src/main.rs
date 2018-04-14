#![feature(io)]

use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::io::BufReader;

/// Returns a `HashMap` of each letter and its count
fn count_chars<I, T: Hash + Eq>(chars: I) -> HashMap<T, usize>
where
    I: Iterator<Item = T>,
{
    // something like `HashMap::with_capacity(chars.len() / 2)` might be worthwile for large inputs
    let mut map = HashMap::new();
    for letter in chars {
        *map.entry(letter).or_insert(0) += 1;
    }
    map
}

fn main() {
    let file = File::open("resources/unixdict.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let chars = reader.chars().map(|c| c.unwrap());
    let count = count_chars(chars);
    println!("{:?}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let map = count_chars("".chars());
        assert!(map.is_empty());
    }

    #[test]
    fn test_basic() {
        let map = count_chars("aaaabbbbc".chars());

        assert_eq!(map.len(), 3);
        assert_eq!(map[&'a'], 4);
        assert_eq!(map[&'b'], 4);
        assert_eq!(map[&'c'], 1);
    }
}
