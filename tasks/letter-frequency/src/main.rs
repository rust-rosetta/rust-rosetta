use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

/// Returns a `HashMap` of each letter and its count
fn count_chars<R: Read>(reader: R) -> io::Result<HashMap<char, usize>> {
    let reader = BufReader::new(reader);

    let mut map = HashMap::new();
    for line in reader.lines() {
        for c in line?.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
    }

    Ok(map)
}

fn main() -> io::Result<()> {
    let file = File::open("resources/unixdict.txt")?;
    let reader = BufReader::new(file);
    let count = count_chars(reader)?;
    println!("{:?}", count);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Cursor;

    #[test]
    fn test_empty() -> io::Result<()> {
        let map = count_chars(Cursor::new(b""))?;
        assert!(map.is_empty());
        Ok(())
    }

    #[test]
    fn test_basic() -> io::Result<()> {
        let map = count_chars(Cursor::new(b"aaaabbbbc"))?;
        assert_eq!(map.len(), 3);
        assert_eq!(map[&'a'], 4);
        assert_eq!(map[&'b'], 4);
        assert_eq!(map[&'c'], 1);
        Ok(())
    }
}
