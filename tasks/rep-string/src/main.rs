fn main() {
    let strings = vec![
        String::from("1001110011"),
        String::from("1110111011"),
        String::from("0010010010"),
        String::from("1010101010"),
        String::from("1111111111"),
        String::from("0100101101"),
        String::from("0100100"),
        String::from("101"),
        String::from("11"),
        String::from("00"),
        String::from("1"),
    ];
    for string in strings {
        match rep_string(&string) {
            Some(rep_string) => println!(
                "Longuest rep-string for '{}' is '{}' ({} chars)",
                string,
                rep_string,
                rep_string.len(),
            ),
            None => println!("No rep-string found for '{}'", string),
        };
    }
}

fn rep_string(string: &str) -> Option<&str> {
    let index = string.len() / 2;

    for split_index in (1..=index).rev() {
        let mut is_rep_string = true;
        let (first, last) = string.split_at(split_index);

        let inter = last.chars().collect::<Vec<char>>();
        let mut iter = inter.chunks_exact(split_index);
        for chunk in iter.by_ref() {
            if first != chunk.iter().collect::<String>() {
                is_rep_string = false;
                break;
            }
        }
        let rmnd = iter.remainder().iter().collect::<String>();

        // Check that the remainder starts with the rep-string
        if !first.starts_with(rmnd.as_str()) {
            is_rep_string = false;
        }

        if is_rep_string {
            return Some(first);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::rep_string;
    use std::collections::HashMap;

    #[test]
    fn test_rep_string() {
        let mut results = HashMap::new();
        results.insert(String::from("1001110011"), Some("10011"));
        results.insert(String::from("1110111011"), Some("1110"));
        results.insert(String::from("0010010010"), Some("001"));
        results.insert(String::from("1010101010"), Some("1010"));
        results.insert(String::from("1111111111"), Some("11111"));
        results.insert(String::from("0100101101"), None);
        results.insert(String::from("0100100"), Some("010"));
        results.insert(String::from("101"), None);
        results.insert(String::from("11"), Some("1"));
        results.insert(String::from("00"), Some("0"));
        results.insert(String::from("1"), None);

        for (input, expected) in results {
            assert_eq!(expected, rep_string(&input));
        }
    }
}
