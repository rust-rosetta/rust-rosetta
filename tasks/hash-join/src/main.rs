use std::collections::HashMap;

type LeftTable<'a> = Vec<(i64, &'a str)>;
type RightTable<'a> = Vec<(&'a str, &'a str)>;
type ResultTable<'a> = Vec<(i64, &'a str, &'a str)>;

fn hash_join<'a>(s: LeftTable<'a>, r: RightTable<'a>) -> ResultTable<'a> {
    let mut hash_map: HashMap<&str, Vec<&str>> = HashMap::new();

    // hash phase
    for (name, nemesis) in r {
        if hash_map.contains_key(name) {
            hash_map.get_mut(name).unwrap().push(nemesis);
        } else {
            hash_map.insert(name, vec![nemesis]);
        }
    }

    let mut result = vec![];
    // join phase
    for (age, name) in s {
        if let Some(v) = hash_map.get(name) {
            for nemesis in v {
                result.push((age, name, *nemesis));
            }
        }
    }
    result
}

pub fn main() {
    let table1 = vec![(27, "Jonah"), (18, "Alan"), (28, "Glory"), (18, "Popeye"), (28, "Alan")];
    let table2 = vec![("Jonah", "Whales"),
                      ("Jonah", "Spiders"),
                      ("Alan", "Ghosts"),
                      ("Alan", "Zombies"),
                      ("Glory", "Buffy")];
    let result = hash_join(table1, table2);
    for (age, name, nemesis) in result {
        println!("{}, {}, {}", age, name, nemesis);
    }
}

#[test]
pub fn test() {
    let t1 = vec![(0, "hello"), (1, "world")];
    let t2 = vec![("hello", "rust"), ("hello", "cargo")];
    let r = hash_join(t1, t2);
    assert!(r == vec![(0, "hello", "rust"), (0, "hello", "cargo")]);
}
