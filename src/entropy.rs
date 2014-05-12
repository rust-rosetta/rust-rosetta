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
// Needed so fibonacci_word compiles cleanly, because fibonacci_word
// uses this code as a library
#[allow(dead_code)]
#[cfg(not(test))]
fn main() {
    println!("{:f}", shannon_entropy("1223334444"));
}


#[test]
fn test_entropy() {
    let tests = vec![
        ("1223334444", 1.846439344671f64),
        ("1223334444555555555", 1.969811065121),
        ("122333", 1.459147917061),
        ("1227774444", 1.846439344671),
        ("aaBBcccDDDD", 1.936260027482),
        ("1234567890abcdefghijklmnopqrstuvwxyz", 5.169925004424),
        ("Rosetta Code", 3.084962500407)];
    // Good enough, actual float epsilon is much smaller
    let epsilon: f64 = 0.0000001;
    for (input, expected) in tests.move_iter() {
        let output = shannon_entropy(input);
        assert!((output - expected).abs() < epsilon);
    }
}
