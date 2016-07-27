extern crate run_length_encoding;

use run_length_encoding::encode;

fn main() {
    let mut s = look_and_say("1");
    for _ in 0..20 {
        println!("{}", s);
        s = look_and_say(&s[..]);
    }
}

fn look_and_say(value: &str) -> String {
    if value.chars().any(|c| !c.is_digit(10)) {
        panic!("this task requires all digits");
    }
    encode(value)
}

#[test]
fn test_say() {
    assert_eq!(look_and_say("11222333"), "213233".to_string());
}
