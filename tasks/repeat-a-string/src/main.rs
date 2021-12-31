fn main() {
    println!("{}", "ha".repeat(5));
}

#[test]
fn check_repeat() {
    assert_eq!("ha".repeat(5), "hahahahaha");
}
