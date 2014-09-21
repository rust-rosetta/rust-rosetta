// http://rosettacode.org/wiki/Repeat_a_string

#[cfg(not(test))]
fn main() {
    println!("{}", "ha".repeat(5).as_slice());
}

#[test]
fn check_repeat() {
    assert!("ha".repeat(5).as_slice() == "hahahahaha");
}
