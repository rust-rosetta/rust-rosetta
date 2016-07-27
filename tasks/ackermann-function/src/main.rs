fn ack(m: isize, n: isize) -> isize {
    if m == 0 {
        n + 1
    } else if n == 0 {
        ack(m - 1, 1)
    } else {
        ack(m - 1, ack(m, n - 1))
    }
}

#[test]
fn test_ack() {
    let expected = 125;
    let found = ack(3, 4);
    assert_eq!(expected, found)
}

fn main() {
    let a = ack(3, 4);
    println!("{}", a);
}
