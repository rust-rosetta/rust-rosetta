use std::fs::File;
use std::io::{BufReader, BufRead};

fn is_ordered(s: &str) -> bool {
    let mut prev = '\x00';
    for c in s.chars() {
        if c < prev {
            return false;
        }
        prev = c;
    }

    true
}

fn find_longest_ordered_words(dict: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    let mut longest_length = 0;

    for s in dict.into_iter() {
        if is_ordered(&s) {
            let n = s.len();
            if n > longest_length {
                longest_length = n;
                result.truncate(0);
            }
            if n == longest_length {
                result.push(s.to_owned());
            }
        }
    }

    result
}

fn main() {
    let lines = BufReader::new(File::open("unixdict.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .collect();

    let longest_ordered = find_longest_ordered_words(lines);

    for s in &longest_ordered {
        println!("{}", s.to_string());
    }
}
