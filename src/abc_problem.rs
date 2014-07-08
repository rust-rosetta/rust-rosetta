// http://rosettacode.org/wiki/ABC_Problem

static words: &'static[&'static str] = &["A", "BARK", "BOOK", "TREAT",
                                    "COMMON", "SQUAD", "CONFUSE", "BARK"];

static blocks: &'static[&'static str] = &["BO", "XK", "DQ", "CP", "NA",
                                          "GT", "RE", "TG", "QD", "FS",
                                          "JW", "HU", "VI", "AN", "OB",
                                          "ER", "FS", "LY", "PC", "ZM"];

#[cfg(not(test))]
fn main() {

    println!("******\nmethod 1\n******");
    for word in words.iter() {
        println!("can {} be built? {}", word, can_be_built_input_first(*word))
    }

    println!("\n******\nmethod 2\n******");

    for word in words.iter() {
        println!("can {} be built? {}", word, can_be_built_blocks_first(*word))
    }
}

// first method (the one that is used in most othe solution)
// iterates on the letters of the input and checks for each letter
// if there's a suitable unused block
fn can_be_built_input_first(input: &str) -> bool {
    let mut used_idx : Vec<uint> = Vec::new();
    let mut final = String::new();

    for chr in input.as_slice().chars() {
        for (ind, &block) in blocks.iter().enumerate() {
            if used_idx.contains(&ind) {continue}
            if block.contains_char(chr) {
                final.push_char(chr);
                used_idx.push(ind);
                break
            }
        }
    }

    // checking if we found all the characters of the original string
    // if so lenghts of the string we built out of the blocks will
    // be the same as the origianal one
    final.len() == input.len()
}

// second method. Iterates on the blocks first. For each block
// it checks if it can be used for one of the letters
// in the input that still need to be composed.
// I find the logic harder to follow vs the first method
// but it should be more efficient (as there must be more blocks than
// letters in the word to compose, and this is iterating on the blocks just once
// and stops as soon as the word is composed,
// while the first method iterates all the blocks for every letter)
fn can_be_built_blocks_first(input: &str) -> bool {
    let mut matched_idx : Vec<(uint, char)> = Vec::new();

    for &block in blocks.iter() {
        'inner: for letter in block.chars() {
            // see if any of the letters in the block can be used for this word
            match input.char_indices().find(|&(i, c)| c==letter &&
                // and that the letter hasn't been already matched by another block
                !matched_idx.iter().any(|&(m_i, _)| i==m_i)) {
                Some((idx,c)) => {
                    // letter with offset idx in the original word has been matched
                    matched_idx.push((idx,c));
                    // don't check the other letter in this block (we can use only one per block)
                    break 'inner
                }
                _ => ()
            };
        }
        if matched_idx.len() == input.len() {
            return true;
        }
    }
    return false;
}

#[test]
fn test_solutions() {
    let expected=[true, true, false, true, false, true, true, true];

    for (&word, &exp) in words.iter().zip(expected.iter()) {
        assert_eq!(can_be_built_input_first(word), exp);
        assert_eq!(can_be_built_blocks_first(word), exp);
    }
}