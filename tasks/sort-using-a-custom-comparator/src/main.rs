fn sort_custom(strings: &mut Vec<&str>) {
    use std::cmp::Ordering;
    strings.sort_by(|a, b| {
        if a.len() > b.len() {
            return Ordering::Less;
        }
        if a.len() < b.len() {
            return Ordering::Greater;
        }
        a.cmp(b)
    });
}

fn main() {
    let mut strings = vec!["Here", "are", "some", "sample", "strings", "to", "be", "sorted"];
    sort_custom(&mut strings);
    println!("{:?}", strings);
}

#[cfg(test)]
mod tests {
    use super::sort_custom;
    #[test]
    fn test_descending_in_length() {
        let mut strings = vec!["a", "aa", "aaa", "aaaa", "aaaaa"];
        sort_custom(&mut strings);
        assert_eq!(strings, ["aaaaa", "aaaa", "aaa", "aa", "a"]);
    }
    #[test]
    fn test_ascending_lexicographically() {
        let mut strings = vec!["baaaa", "abaaa", "aabaa", "aaaba", "aaaab"];
        sort_custom(&mut strings);
        assert_eq!(strings, ["aaaab", "aaaba", "aabaa", "abaaa", "baaaa"]);
    }
    #[test]
    fn test_mixture() {
        let mut strings = vec!["a", "A", "ba", "aa", "AA", "aAa", "aaa"];
        sort_custom(&mut strings);
        assert_eq!(strings, ["aAa", "aaa", "AA", "aa", "ba", "A", "a"]);
    }
}
