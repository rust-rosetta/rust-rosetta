fn main() {
    println!(
        "{}",
        strip_characters("She was a soul stripper. She took my heart!", "aei")
    );
}

fn strip_characters(original: &str, to_strip: &str) -> String {
    original
        .chars()
        .filter(|&c| !to_strip.contains(c))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_strip_characters() {
        let test = "She was a soul stripper. She took my heart!";
        let expected = "Sh ws  soul strppr. Sh took my hrt!";
        assert_eq!(strip_characters(test, "aei"), expected);
    }

    #[test]
    fn correct_strip_empty_string() {
        assert_eq!(strip_characters("", "jkl"), "");
    }

    #[test]
    fn correct_strip_no_characters() {
        assert_eq!(strip_characters("test string", ""), "test string");
    }
}
