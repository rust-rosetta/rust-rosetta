use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn sorted_characters(string: &str) -> String {
    let mut chars = string.chars().collect::<Vec<char>>();
    chars.sort();
    chars.iter().cloned().collect()
}

/// Returns groups of anagrams where each group consists of a set
/// containing the words
fn anagrams<T: Iterator<Item = String>>(lines: T) -> HashMap<String, HashSet<String>> {
    let mut groups = HashMap::new();

    // Make groups of words according to the letters they contain
    for line in lines {
        let s = line.trim();
        let sorted = sorted_characters(s);
        let set = match groups.entry(sorted) {
            Vacant(entry) => entry.insert(HashSet::new()), // Insert new set if not found
            Occupied(entry) => entry.into_mut(),
        };

        set.insert(s.to_string());
    }

    groups
}

/// Returns the groups of anagrams that contain the most words in them
fn largest_groups(groups: &HashMap<String, HashSet<String>>) -> HashMap<String, HashSet<String>> {
    let max_length = groups.iter()
        .map(|(_, group)| group.len())
        .max()
        .unwrap();
    groups.iter()
        .filter_map(|(key, group)| {
            if group.len() == max_length {
                Some((key.clone(), group.clone()))
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let reader = BufReader::new(File::open("resources/unixdict.txt").unwrap());
    let lines = reader.lines().map(|l| l.unwrap());

    let anagram_groups = anagrams(lines);
    let largest_groups = largest_groups(&anagram_groups);

    // Print the words in the largest groups of anagrams
    for group in largest_groups.values() {
        for word in group {
            print!("{} ", word)
        }
        println!("")
    }
}

#[test]
fn basic_test() {
    fn to_hash(s: &[&str]) -> HashSet<String> {
        s.iter().map(|s| s.to_string()).collect()
    };

    fn assert_has_value(map: &HashMap<String, HashSet<String>>, set: &HashSet<String>) {
        assert!(map.values().any(|v| v == set));
    }

    // Groups of anagrams
    let group1 = &to_hash(&["lane", "neal", "lean"]);
    let group2 = &to_hash(&["angel", "angle", "galen"]);
    let group3 = &to_hash(&["glare", "large"]);

    // Prepare the input for the program
    let word_iter = group1.iter().chain(group2.iter().chain(group3.iter()));

    // Here begins the real testing
    let all_groups = &anagrams(word_iter.map(|s| s.to_string()));
    let largest_groups = &largest_groups(all_groups);

    // Groups 1, 2 and 3 are contained in "all_groups"
    assert_has_value(all_groups, group1);
    assert_has_value(all_groups, group2);
    assert_has_value(all_groups, group3);

    // Groups 1 and 2 are contained in "largest_groups". Group 3 is not.
    assert_has_value(largest_groups, group1);
    assert_has_value(largest_groups, group2);
    assert!(largest_groups.values().all(|group| group != group3));
}
