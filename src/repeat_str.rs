// http://rosettacode.org/wiki/Repeat_a_string

#[cfg(not(test))]
fn main() {
    println!("{}", "ha".repeat(5));
}

#[test]
fn check_repeat() {
    assert_eq!("ha".repeat(5), "hahahahaha");
}
