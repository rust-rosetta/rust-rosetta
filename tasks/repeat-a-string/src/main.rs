use std::iter::repeat;

fn main() {
    println!("{}", repeat("ha").take(5).collect::<String>());
}

#[test]
fn check_repeat() {
    assert_eq!(repeat("ha").take(5).collect::<String>(), "hahahahaha");
}
