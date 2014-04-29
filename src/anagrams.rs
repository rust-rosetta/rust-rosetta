// Implements http://rosettacode.org/wiki/Anagrams

extern crate collections;

use std::str;
use collections::HashMap;
use std::io::File;
use std::io::BufferedReader;
use std::cmp;

fn sort_string(string: &str) -> ~str {
	let mut chars: ~[char] = string.chars().collect();
	chars.sort();
	str::from_chars(chars)
}

fn main () {
	let path = Path::new("resources/unixdict.txt");
	let mut file = BufferedReader::new(File::open(&path));

	let mut map: HashMap<~str, Vec<~str>> = HashMap::new();

	for line in file.lines() {
		let s = line.unwrap().trim().to_owned();
		map.insert_or_update_with(sort_string(s.clone()), vec!(s.clone()),
				   |_k, v| v.push(s.clone())
				);
	}

	let max_length = map.iter().fold(0, |s, (_k, v)| cmp::max(s, v.len()));

	for (_k, v) in map.iter() {
		if v.len() == max_length {
			for s in v.iter() {
				print!("{} ", *s)
			}
			println!("")
		}
	}
}
