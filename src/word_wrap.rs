// http://rosettacode.org/wiki/Word_wrap

// Implements the minimum length greedy algorithm
// http://en.wikipedia.org/wiki/Word_wrap#Minimum_length
fn word_wrap(text: &str, line_length: uint) -> String {
    let mut wrapped = String::new();
    let mut space_left = line_length;
    let space_width = 1u;

    for word in text.words() {
        let word_length = word.char_len();

        if space_left != line_length {
            if word_length + space_width > space_left {
                wrapped.push('\n');
                space_left = line_length;
            }
            else {
                wrapped.push(' ');
                space_left -= space_width;
            }
        }

        wrapped.push_str(word);
        space_left =
            if word_length > line_length { 0 } else { space_left - word_length }
    }

    wrapped
}

#[cfg(not(test))]
fn main () {
    let text =
        "In olden times when wishing still helped one, there lived a king \
         whose daughters were all beautiful, but the youngest was so beautiful \
         that the sun itself, which has seen so much, was astonished whenever \
         it shone in her face.  Close by the king's castle lay a great dark \
         forest, and under an old lime tree in the forest was a well, and when \
         the day was very warm, the king's child went out into the forest and \
         sat down by the side of the cool fountain, and when she was bored she \
         took a golden ball, and threw it up on high and caught it, and this \
         ball was her favorite plaything.";

    for &length in [72u, 80u].iter() {
        println!("Text wrapped at {}:\n{}\n", length, word_wrap(text, length));
    }
}

#[test]
fn test_empty_string() {
    assert_eq!(word_wrap("", 80).as_slice(), "");
}

#[test]
fn test_single_word_shorter_than_line() {
    assert_eq!(word_wrap("Hello", 80).as_slice(), "Hello");
}

#[test]
fn test_two_words_shorter_than_line() {
    assert_eq!(word_wrap("Hello world", 80).as_slice(), "Hello world");
}

#[test]
fn test_single_word_longer_than_line() {
    assert_eq!(word_wrap("Hello", 4).as_slice(), "Hello");
}

#[test]
fn test_wrap_second_word() {
    assert_eq!(word_wrap("Hello world", 10).as_slice(), "Hello\nworld");
}

#[test]
fn test_wrap_all_as_words_shorter_than_line_length() {
    assert_eq!(word_wrap("Words frequently wrapped", 4).as_slice(),
               "Words\nfrequently\nwrapped");
}

#[test]
fn test_wrap_punctuation() {
    assert_eq!(word_wrap("Hello, world", 6).as_slice(), "Hello,\nworld");
}

#[test]
fn test_squash_multiple_spaces() {
    assert_eq!(word_wrap(" Hello  to the    world    ", 10).as_slice(),
               "Hello to\nthe world");
}

#[test]
fn test_unicode() {
    assert_eq!(word_wrap("Nous étions à l'Étude, quand le Proviseur entra",
                         11).as_slice(),
               "Nous étions\nà l'Étude,\nquand le\nProviseur\nentra");
}
