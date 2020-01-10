fn reverse_string(string: &str) -> String {
    string.chars().rev().collect::<String>()
}

fn reverse_words(string: &str) -> String {
    string
        .split_whitespace()
        .map(|x| x.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

fn reverse_word_order(string: &str) -> String {
    string
        .split_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string() {
        let string = "rosetta code phrase reversal";
        assert_eq!(
            reverse_string(string.clone()),
            "lasrever esarhp edoc attesor"
        );
    }

    #[test]
    fn test_reverse_words() {
        let string = "rosetta code phrase reversal";
        assert_eq!(
            reverse_words(string.clone()),
            "attesor edoc esarhp lasrever"
        );
    }

    #[test]
    fn test_reverse_word_order() {
        let string = "rosetta code phrase reversal";
        assert_eq!(
            reverse_word_order(string.clone()),
            "reversal phrase code rosetta"
        );
    }
}
