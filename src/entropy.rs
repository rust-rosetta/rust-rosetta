// Implements http://rosettacode.org/wiki/Entropy
extern crate collections;

use std::str::StrSlice;
use collections::hashmap::HashMap;

pub fn shannon_entropy(s: &str) -> f64 {
    let mut map = HashMap::<char, uint>::new();

    // Count occurrences of each char
    for c in s.chars() {
        map.insert_or_update_with(c, 1, |_,v| *v += 1);
    }

    // Calculate the entropy
    let len = s.len() as f64;
    map.iter().fold(0f64, |mut acc, (_, nb)| {
        let p = (*nb as f64) / len;
        acc -= p * p.log2();
        acc
    })
}

#[cfg(not(test))]
fn main() {
    println!("{:f}", shannon_entropy("1223334444"));
}
