use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

pub fn shannon_entropy(s: &str) -> f64 {
    let mut map = HashMap::new();

    // Count occurrences of each char
    for c in s.chars() {
        match map.entry(c) {
            Vacant(entry) => {
                entry.insert(1);
            }
            Occupied(mut entry) => {
                *entry.get_mut() += 1;
            }
        };
    }

    // Calculate the entropy
    let len = s.len() as f64;
    map.iter().fold(0f64, |acc, (_, &nb)| {
        let p = nb as f64 / len;
        acc - p * p.log2()
    })
}

#[test]
fn test_entropy() {
    let tests = vec![("1223334444", 1.846439344671f64),
                     ("1223334444555555555", 1.969811065121),
                     ("122333", 1.459147917061),
                     ("1227774444", 1.846439344671),
                     ("aaBBcccDDDD", 1.936260027482),
                     ("1234567890abcdefghijklmnopqrstuvwxyz", 5.169925004424),
                     ("Rosetta Code", 3.084962500407)];
    // Good enough, actual float epsilon is much smaller
    let epsilon: f64 = 0.0000001;
    for (input, expected) in tests {
        let output = shannon_entropy(input);
        assert!((output - expected).abs() < epsilon);
    }
}
