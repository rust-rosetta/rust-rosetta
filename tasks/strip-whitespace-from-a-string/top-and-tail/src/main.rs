fn main() {
    let string = "String without spaces";
    let spaces = " \x0B\t\r\n \u{A0} \u{2000} \u{3000} ";
    let string_with_spaces = spaces.to_string() + string + spaces;

    assert_eq!(string_with_spaces.trim(), string);
    assert_eq!(string_with_spaces.trim_left().to_string(),
               string.to_string() + spaces);
    assert_eq!(string_with_spaces.trim_right().to_string(),
               spaces.to_string() + string);
}
