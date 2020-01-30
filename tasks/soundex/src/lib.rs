use std::collections::HashMap;

fn soundex(word: &str) -> String {
    let dropped_vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
    let mut consonants_value = HashMap::new();

    let letter_groups = vec![
        vec!['b', 'f', 'p', 'v'],
        vec!['c', 'g', 'j', 'k', 'q', 's', 'x', 'z'],
        vec!['d', 't'],
        vec!['l'],
        vec!['m', 'n'],
        vec!['r'],
    ];
    for (idx, letter_group) in letter_groups.iter().enumerate() {
        for ch in letter_group {
            // so we have '1', '2', etc chars instead of numbers
            consonants_value.insert(ch, (b'1' + idx as u8) as char);
        }
    }

    let soundex = String::from(word);
    let mut soundex_numbers = {
        soundex
            .chars()
            .enumerate()
            .map(|(_, ch)| {
                if consonants_value.contains_key(&ch) {
                    consonants_value[&ch]
                } else {
                    ch
                }
            })
            .collect::<Vec<char>>()
    };
    soundex_numbers.dedup();
    let mut soundex_numbers = {
        soundex_numbers
            .iter()
            .enumerate()
            .filter(|(idx, ch)| {
                if *idx == 0 {
                    return true;
                } else if dropped_vowels.contains(ch) {
                    return false;
                }
                true
            })
            .map(|(_, ch)| *ch)
            .collect::<Vec<char>>()
    };

    // deals with h and w
    while let Some(index) = soundex_numbers.iter().position(|&x| x == 'h' || x == 'w') {
        soundex_numbers.remove(index);
        if let Some(before) = soundex_numbers.get(index - 1) {
            if let Some(after) = soundex_numbers.get(index) {
                if before == after {
                    soundex_numbers.remove(index);
                }
            }
        }
    }

    while soundex_numbers.len() < 4 {
        soundex_numbers.push('0');
    }
    soundex_numbers[..4].iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soundex() {
        assert_eq!("A261", soundex("Ashcraft"));
        assert_eq!("T522", soundex("Tymczak"));
        assert_eq!("B625", soundex("Baragwanath"));
        assert_eq!("G362", soundex("Gutierrez"));
        assert_eq!("M220", soundex("Moses"));
    }
}
