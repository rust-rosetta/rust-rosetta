extern crate regex;

use std::io;
use std::io::prelude::*;

use regex::Regex;

fn find_bare_lang_tags(input: &str) -> Vec<(Option<String>, i32)> {
    let mut language_pairs = vec![];
    let mut language = None;
    let mut counter = 0_i32;

    let header_re = Regex::new(r"==\{\{header\|(?P<lang>[[:alpha:]]+)\}\}==").unwrap();

    for line in input.lines() {
        if let Some(captures) = header_re.captures(line) {
            if let Some(header_lang) = captures.name("lang") {
                language_pairs.push((language, counter));
                language = Some(header_lang.as_str().to_owned());
                counter = 0;
            }
        }

        if line.contains("<lang>") {
            counter += 1;
        }
    }

    language_pairs.push((language, counter));
    language_pairs
}

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.lock().read_to_string(&mut buf).unwrap();
    let results = find_bare_lang_tags(&buf);
    let total_bare = results.iter().map(|r| r.1).sum::<i32>();

    println!("{} bare language tags.\n", total_bare);
    for result in &results {
        let num_bare = result.1;

        if num_bare > 0 {
            println!("{} in {}",
                     result.1,
                     result.0.to_owned().unwrap_or("no language".to_owned()));
        }
    }
}

#[test]
fn test_bare_tags() {
    let input = r#"
    Description
    <lang>Pseudocode</lang>

    =={{header|C}}==
    <lang C>printf("Hello world!\n");</lang>

    =={{header|Perl}}==
    <lang>print "Hello world!\n"</lang>"#;

    let expected = vec![
        (None, 1),
        (Some("C".to_owned()), 0),
        (Some("Perl".to_owned()), 1),
    ];

    assert_eq!(expected, find_bare_lang_tags(&input));
}
