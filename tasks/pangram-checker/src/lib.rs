use std::collections::HashSet;

pub fn is_pangram_via_bitmask(s: &str) -> bool {
    // Create a mask of set bits and convert to false as we find characters.
    let mut mask = (1 << 26) - 1;

    for chr in s.chars() {
        let val = chr as u32 & !0x20; /* 0x20 converts lowercase to upper */
        if val <= 'Z' as u32 && val >= 'A' as u32 {
            mask &= !(1 << (val - 'A' as u32));
        }
    }

    mask == 0
}

pub fn is_pangram_via_hashset(s: &str) -> bool {
    // Insert lowercase letters into a HashSet, then check if we have at least 26.
    let letters = s
        .chars()
        .flat_map(|chr| chr.to_lowercase())
        .filter(|&chr| ('a'..='z').contains(&chr))
        .fold(HashSet::new(), |mut letters, chr| {
            letters.insert(chr);
            letters
        });

    letters.len() == 26
}

pub fn is_pangram_via_sort(s: &str) -> bool {
    // Copy chars into a vector, convert to lowercase, sort, and remove duplicates.
    let mut chars: Vec<char> = s
        .chars()
        .flat_map(|chr| chr.to_lowercase())
        .filter(|&chr| ('a'..='z').contains(&chr))
        .collect();

    chars.sort_unstable();
    chars.dedup();

    chars.len() == 26
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        assert!(is_pangram_via_sort(
            "The quick brown fox jumps over the lazy dog"
        ));
        assert!(!is_pangram_via_sort(
            "The quick white cat jumps over the lazy dog"
        ));
    }

    #[test]
    fn test_bitmask() {
        assert!(is_pangram_via_bitmask(
            "The quick brown fox jumps over the lazy dog"
        ));
        assert!(!is_pangram_via_bitmask(
            "The quick white cat jumps over the lazy dog"
        ));
    }

    #[test]
    fn test_hashset() {
        assert!(is_pangram_via_hashset(
            "The quick brown fox jumps over the lazy dog"
        ));
        assert!(!is_pangram_via_hashset(
            "The quick white cat jumps over the lazy dog"
        ));
    }
}
