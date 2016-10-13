
#![feature(test)]

extern crate test;

pub fn is_pangram_via_bitmask(s: &str) -> bool {

    // Create a mask of set bits and convert to false as we find characters.
    let mut mask = (1 << 26) - 1;

    for chr in s.chars() {
        let val = chr as u32 & !0x20; /* 0x20 converts lowercase to upper */
        if val <= 'Z' as u32 && val >= 'A' as u32 {
            mask = mask & !(1 << (val - 'A' as u32));
        }
    }

    mask == 0
}

pub fn is_pangram_via_sort(s: &str) -> bool {

    // Copy chars into a vector, sort and remove duplicates
    let mut chars: Vec<char> = s.to_lowercase()
        .chars()
        .filter(|&chr| chr >= 'a' && chr <= 'z')
        .collect();

    chars.sort();
    chars.dedup();

    chars.len() == 26
}

fn main() {

    let examples = ["The quick brown fox jumps over the lazy dog",
                    "The quick white cat jumps over the lazy dog"];

    for &text in examples.iter() {
        let is_pangram = is_pangram_via_sort(text);
        println!("Is \"{}\" a pangram via sort? - {}", text, is_pangram);
    }

    for &text in examples.iter() {
        let is_pangram = is_pangram_via_bitmask(text);
        println!("Is \"{}\" a pangram via bitmask? - {}", text, is_pangram);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_sort() {
        let result = is_pangram_via_sort("The quick brown fox jumps over the lazy dog");
        assert!(result);

        let result2 = is_pangram_via_sort("The quick white cat jumps over the lazy dog");
        assert!(!result2);
    }

    #[test]
    fn test_bitmask() {
        let result1 = is_pangram_via_bitmask("The quick brown fox jumps over the lazy dog");
        assert!(result1);

        let result2 = is_pangram_via_bitmask("The quick white cat jumps over the lazy dog");
        assert!(!result2);
    }

    #[bench]
    fn sort_speed(b: &mut Bencher) {
        b.iter(|| is_pangram_via_sort("The quick white cat jumps over the lazy dog"));
    }

    #[bench]
    fn bitmask_speed(b: &mut Bencher) {
        b.iter(|| is_pangram_via_bitmask("The quick white cat jumps over the lazy dog"));
    }

}
