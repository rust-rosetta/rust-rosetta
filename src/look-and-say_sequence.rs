// http://rosettacode.org/wiki/Look-and-say_sequence
use run_length_encoding::encode;

mod run_length_encoding;

#[cfg(not(test))]
fn main() {
    let mut s = look_and_say("1");
    for _ in range(0u,20) {
        println!("{}", s);
        s = look_and_say(s.as_slice());
    }
}

fn look_and_say(value: &str) -> String {
    if value.chars().any(|c| !UnicodeChar::is_numeric(c)) { panic!("this task requires all digits"); }
    encode(value)
}

#[test]
fn test_say() {
    assert_eq!(look_and_say("11222333"),"213233".to_string());
}
