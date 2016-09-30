fn rev_words(line: &str) -> String {
    line.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
}

fn rev_words_on_lines(text: &str) -> String {
    text.lines().map(rev_words).collect::<Vec<String>>().join("\n")
}

fn main() {
    let text = r"---------- Ice and Fire ------------

fire, in end will world the say Some
ice. in say Some
desire of tasted I've what From
fire. favor who those with hold I

... elided paragraph last ...

Frost Robert -----------------------";

    println!("{}", rev_words_on_lines(text));
}

#[test]
fn test_rev_words() {
    let tests = [("", ""),
                 ("a", "a"),
                 ("a b", "b a"),
                 ("cat dog", "dog cat"),
                 // According to the problem, multiple spaces can be
                 // compressed into a single space.
                 ("cat     dog", "dog cat"),
                 ("cat dog frog", "frog dog cat")];

    for &(input, expected) in &tests {
        let output = rev_words(input);
        assert_eq!(expected, output);
    }
}

#[test]
fn test_rev_words_on_lines() {
    // The tests from test_rev_words should have the same results, so
    // we include them.
    let tests = [("", ""),
                 ("a", "a"),
                 ("a b", "b a"),
                 ("cat dog", "dog cat"),
                 // According to the problem, multiple spaces can be
                 // compressed into a single space.
                 ("cat     dog", "dog cat"),
                 ("cat dog frog", "frog dog cat"),

                 // Multiple Lines
                 ("a b\nb a", "b a\na b"),
                 ("a b\nc d\ne f", "b a\nd c\nf e")];

    for &(input, expected) in &tests {
        let output = rev_words_on_lines(input);
        assert_eq!(expected, output);
    }
}
