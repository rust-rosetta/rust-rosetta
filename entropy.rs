// Implements http://rosettacode.org/wiki/Entropy

use std::num::Real;
use std::hashmap::HashMap;
use std::str::StrSlice;

pub fn shannon_entropy(s: &str) -> f64 {
    let mut map = HashMap::<char, uint>::new();

    for c in s.chars() {
        map.insert_or_update_with(c, 1, |_,v| *v += 1);
    }

    map.iter().fold(0f64, |mut acc, (_, nb)| {
        let p = (*nb as f64)/(s.len() as f64);
        acc -= p * p.log2(); 
        acc
    })
}

fn main() {
    println!("{:f}", shannon_entropy("1223334444"));
}
