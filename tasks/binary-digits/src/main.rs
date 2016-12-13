trait BinaryString {
    fn to_binary_string(&self) -> String;
}

impl BinaryString for usize {
    fn to_binary_string(&self) -> String {
        format!("{:b}", *self)
    }
}

fn main() {
    for s in 0..17 {
        println!("{}", s.to_binary_string());
    }
}

#[test]
fn test_digits() {
    let expected = ["0", "1", "10", "11", "100", "101", "110", "111", "1000", "1001", "1010",
                    "1011", "1100", "1101", "1110", "1111"];

    for (n, expected) in (0..17).zip(expected.iter()) {
        assert_eq!(n.to_binary_string(), *expected);
    }
}
