use std::collections::HashMap;
use std::hash::Hash;

// If you know one of the tables is smaller, it is best to make it the second parameter.
fn hash_join<A, B, K>(first: &[(K, A)], second: &[(K, B)]) -> Vec<(A, K, B)>
where
    K: Hash + Eq + Copy,
    A: Copy,
    B: Copy,
{
    let mut hash_map = HashMap::new();

    // hash phase
    for &(key, val_a) in second {
        // collect all values by their keys, appending new ones to each existing entry
        hash_map.entry(key).or_insert_with(Vec::new).push(val_a);
    }

    let mut result = Vec::new();
    // join phase
    for &(key, val_b) in first {
        if let Some(vals) = hash_map.get(&key) {
            let tuples = vals.iter().map(|&val_a| (val_b, key, val_a));
            result.extend(tuples);
        }
    }

    result
}

fn main() {
    let table1 = [
        ("Jonah", 27),
        ("Alan", 18),
        ("Glory", 28),
        ("Popeye", 18),
        ("Alan", 28),
    ];
    let table2 = [
        ("Jonah", "Whales"),
        ("Jonah", "Spiders"),
        ("Alan", "Ghosts"),
        ("Alan", "Zombies"),
        ("Glory", "Buffy"),
    ];
    let result = hash_join(&table1, &table2);
    println!("Age | Character Name | Nemesis");
    println!("----|----------------|--------");
    for (age, name, nemesis) in result {
        println!("{:<3} | {:^14} | {}", age, name, nemesis);
    }
}

#[test]
fn test() {
    let t1 = [("hello", 0), ("world", 1)];
    let t2 = [("hello", "rust"), ("hello", "cargo")];
    let r = hash_join(&t1, &t2);
    let expected = [(0, "hello", "rust"), (0, "hello", "cargo")];
    assert_eq!(r, expected);
}
