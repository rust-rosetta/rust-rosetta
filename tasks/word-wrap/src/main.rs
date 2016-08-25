//! Using the [minimum length greedy algorithm].
//!
//! Implemented as a lazy `String` iterator, returning a wrapped line each time.
//!
//! [minimum length greedy algorithm]: http://en.wikipedia.org/wiki/Word_wrap#Minimum_length
use std::mem::swap;
use std::str::SplitWhitespace;

pub struct WordWrap<'a> {
    words: SplitWhitespace<'a>,
    line_length: usize,
    next_line: String,
}

impl<'a> WordWrap<'a> {
    fn new(text: &'a str, line_length: usize) -> WordWrap {
        WordWrap {
            words: text.split_whitespace(),
            line_length: line_length,
            next_line: String::new(),
        }
    }
}

impl<'a> Iterator for WordWrap<'a> {
    type Item = String;

    #[cfg_attr(feature="clippy", allow(while_let_on_iterator))]
    fn next(&mut self) -> Option<String> {
        // Move anything left over from last run to this_line
        let mut this_line = String::new();
        swap(&mut self.next_line, &mut this_line);

        let mut space_left = self.line_length - this_line.chars().count();
        const SPACE_WIDTH: usize = 1;

        // Loop, adding words until we run out of words or hit the line length
        while let Some(word) = self.words.next() {
            let word_length = word.chars().count();

            // If not the first word for this line
            if space_left != self.line_length {
                if word_length + SPACE_WIDTH > space_left {
                    // Out of space, save word for next line
                    self.next_line.push_str(word);
                    break;
                } else {
                    // Add a space and keep going
                    this_line.push(' ');
                    space_left -= SPACE_WIDTH;
                }
            }

            // Add word to this line
            this_line.push_str(word);
            space_left -= word_length;
        }

        if this_line.is_empty() {
            None
        } else {
            Some(this_line)
        }
    }
}

fn main() {
    let text = "In olden times when wishing still helped one, there lived a king whose daughters \
                were all beautiful, but the youngest was so beautiful that the sun itself, which \
                has seen so much, was astonished whenever it shone in her face.  Close by the \
                king's castle lay a great dark forest, and under an old lime tree in the forest \
                was a well, and when the day was very warm, the king's child went out into the \
                forest and sat down by the side of the cool fountain, and when she was bored she \
                took a golden ball, and threw it up on high and caught it, and this ball was her \
                favorite plaything.";

    for length in 72..81 {
        println!("Text wrapped at {}", length);
        for line in WordWrap::new(text, length) {
            println!("{}", line);
        }
        println!("");
    }
}

#[test]
fn test_empty_string() {
    assert_eq!(WordWrap::new("", 80).next(), None);
}

#[test]
fn test_single_word_shorter_than_line() {
    assert_eq!(WordWrap::new("Hello", 80).next().unwrap(), "Hello");
}

#[test]
fn test_two_words_shorter_than_line() {
    assert_eq!(WordWrap::new("Hello world", 80).next().unwrap(),
               "Hello world");
}

#[test]
fn test_wrap_second_word() {
    let mut w = WordWrap::new("Hello world", 10);
    assert_eq!(w.next().unwrap(), "Hello");
    assert_eq!(w.next().unwrap(), "world");
}

#[test]
fn test_wrap_punctuation() {
    let mut w = WordWrap::new("Hello, world", 6);
    assert_eq!(w.next().unwrap(), "Hello,");
    assert_eq!(w.next().unwrap(), "world");
}

#[test]
fn test_squash_multiple_spaces() {
    let mut w = WordWrap::new(" Hello  to the    world    ", 10);
    assert_eq!(w.next().unwrap(), "Hello to");
    assert_eq!(w.next().unwrap(), "the world");
    assert_eq!(w.next(), None);
}

#[test]
fn test_unicode() {
    let mut w = WordWrap::new("Nous étions à l'Étude, quand le Proviseur entra", 11);
    assert_eq!(w.next().unwrap(), "Nous étions");
    assert_eq!(w.next().unwrap(), "à l'Étude,");
    assert_eq!(w.next().unwrap(), "quand le");
    assert_eq!(w.next().unwrap(), "Proviseur");
    assert_eq!(w.next().unwrap(), "entra");
}
