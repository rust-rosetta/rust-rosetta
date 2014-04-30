// Implements http://rosettacode.org/wiki/Anagrams

extern crate collections;

use collections::HashMap;
use std::str;
use std::io::{File, BufferedReader};
use std::cmp::max;

fn sort_string(string: &str) -> ~str {
	let mut chars: ~[char] = string.chars().collect();
	chars.sort();
	str::from_chars(chars)
}

// Returns groups of anagrams (each group consists of a vector containing the words)
fn get_anagrams<T: Reader>(mut reader: BufferedReader<T>) -> HashMap<~str, Vec<~str>> {
    let mut map: HashMap<~str, Vec<~str>> = HashMap::new();

    // Make groups of words according to the letters they contain
    // i.e. evil, live would be in the same group because they share the same letters
	for line in reader.lines() {
		let s = line.unwrap().trim().to_owned();
		map.insert_or_update_with(sort_string(s.clone()), vec!(s.clone()),
				   |_k, v| v.push(s.clone())
				);
	}
    
    map
}

// Returns the groups of anagrams that contain the most words in them
fn get_biggest_groups_anagrams(groups: &HashMap<~str, Vec<~str>>) -> HashMap<~str, Vec<~str>> {
    let max_length = groups.iter().fold(0, |s, (_, v)| max(s, v.len())); 
    
    groups.iter().filter(|&(_, v)| v.len() == max_length).map(|(x, y)| (x.clone(), y.clone())).collect()
}

fn main () {
	let path = Path::new("resources/unixdict.txt");
	let reader = BufferedReader::new(File::open(&path));

	let map = get_anagrams(reader);
	let biggest_groups = get_biggest_groups_anagrams(&map);

	for (_, v) in biggest_groups.iter() {
        for s in v.iter() {
            print!("{} ", *s)
        }
        println!("")
	}
}
