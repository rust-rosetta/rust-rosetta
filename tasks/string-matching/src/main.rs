fn match_string(container: &str, target: &str) -> (bool, bool, bool) {
    let starts = container.starts_with(target);
    let ends = container.ends_with(target);
    let contains = starts || ends || container.contains(target);

    (starts, contains, ends)
}

fn print_info(container: &str, target: &str) {
    println!(r#"Matching "{}" in the string "{}""#, target, container);
    let (starts, contains, ends) = match_string(container, target);

    if starts {
        println!(r#""{}" starts with "{}""#, container, target);
    }
    if contains {
        println!(r#""{}" contains "{}""#, container, target);
    }
    if ends {
        println!(r#""{}" ends with "{}""#, container, target);
    }
}

fn main() {
    print_info("abcd", "ab");
    print_info("abcd", "bc");
    print_info("abcd", "cd");
}

#[test]
fn test_match_string() {
    assert_eq!(match_string("abcd", "ab"), (true, true, false));
    assert_eq!(match_string("abcd", "ba"), (false, false, false));
    assert_eq!(match_string("abcd", "bc"), (false, true, false));
    assert_eq!(match_string("abcd", "cd"), (false, true, true));
}
