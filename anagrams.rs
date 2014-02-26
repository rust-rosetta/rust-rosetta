extern crate collections;

use std::str;
use collections::HashMap;
use std::io::File;
use std::io::BufferedReader;

fn sort_string(string: ~str) -> ~str {
	let mut chars = string.to_utf16();
	chars.sort();
	str::from_utf16(chars).unwrap()
}

fn main () {
	let path = Path::new("unixdict.txt");
	let mut file = BufferedReader::new(File::open(&path));

	let mut map: HashMap<~str, ~[~str]> = HashMap::new();

	for line in file.lines() {
		let s = line.trim().to_owned();
		map.mangle(sort_string(s.clone()), s,
				   |_k, v| ~[v],
				   |_k, v, string| v.push(string)
				);//, sort_string(line))
	}

	let mut max_length = 0;
	for (_k, v) in map.iter() {
		if v.len() > max_length {
			max_length = v.len()
		}
	}

	for (_k, v) in map.iter() {
		if v.len() == max_length {
			for s in v.iter() {
				print!("{} ", *s)
			}
			println!("")
		}
	}
}