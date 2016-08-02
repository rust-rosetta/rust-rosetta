use std::collections::HashSet;

const WORDS: &'static [&'static str] = &["A", "BARK", "BOOK", "TREAT", "COMMON", "SQUAD",
                                         "CONFUSE"];

const BLOCKS: &'static [&'static str] = &["BO", "XK", "DQ", "CP", "NA", "GT", "RE", "TG", "QD",
                                          "FS", "JW", "HU", "VI", "AN", "OB", "ER", "FS", "LY",
                                          "PC", "ZM"];

fn main() {
    println!("******\nmethod 1\n******");
    for word in WORDS {
        println!("can {} be built? {}", word, can_be_built_input_first(*word))
    }

    println!("\n******\nmethod 2\n******");
    for word in WORDS {
        println!("can {} be built? {}",
                 word,
                 can_be_built_blocks_first(*word))
    }
}

/// First method (common solution among other languages)
///
/// Iterates through the letters of the input and checks if there is a suitable
/// *unused* block for each letter
fn can_be_built_input_first(input: &str) -> bool {
    let mut used = HashSet::new();

    for chr in input.chars() {
        for (ind, block) in BLOCKS.iter().enumerate() {
            if !used.contains(&ind) && block.contains(chr) {
                used.insert(ind);
                break;
            }
        }
    }

    // If the number of used indices is equal to the input length, then we
    // have can successfully make the word with the blocks.
    used.len() == input.len()
}

/// Second method
///
/// Iterates over the blocks. For each block it checks if it can be used for one
/// of the characters in the input that hasn't already been fulfilled.
/// It should be more efficient than the first method whenever there are more
/// blocks than characters in the input.
fn can_be_built_blocks_first(input: &str) -> bool {
    let mut matched = HashSet::new();

    BLOCKS.iter().any(|block| {
        for letter in block.chars() {
            let needle = input.char_indices().find(|&(i, c)| {
                // See if any of the letters in the block can be used for this word
                c == letter && !matched.contains(&i)
            });

            if let Some((idx, _)) = needle {
                // letter with offset idx in the original word has been matched
                matched.insert(idx);
                // don't check the other letter in this block
                // (we can use one character per block)
                break;
            }
        }

        // The iterator will halt if/when this becomes true, so it will early
        // return whenever there is a solution found.
        matched.len() == input.len()
    })
}

#[test]
fn test_solutions() {
    let expected = [true, true, false, true, false, true, true];

    for (&word, &exp) in WORDS.iter().zip(expected.iter()) {
        assert_eq!(can_be_built_input_first(word), exp);
        assert_eq!(can_be_built_blocks_first(word), exp);
    }
}
