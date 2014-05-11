// http://rosettacode.org/wiki/Repeat_a_string

#[test]
fn check_repeat() {
    assert!("ha".repeat(5).as_slice() == "hahahahaha");
}