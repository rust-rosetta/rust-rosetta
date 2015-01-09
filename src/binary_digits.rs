// Implements http://rosettacode.org/wiki/Binary_digits
use std::iter::range_inclusive;

trait BinaryString {
    fn to_binary_string(&self) -> String;
}

impl BinaryString for uint {
    fn to_binary_string(&self) -> String {
        format!("{:b}", *self)
    }
}

#[cfg(not(test))]
fn main() {
    for s in range_inclusive(0, 16u) {
        println!("{:?}", s.to_binary_string());
    }
}

#[test]
fn test_digits() {
    let expected = ["0", "1", "10", "11",
                    "100", "101", "110", "111",
                    "1000", "1001", "1010", "1011",
                    "1100", "1101", "1110", "1111"];

    for (n, expected) in range_inclusive(0, 16u).zip(expected.iter()) {
        assert_eq!(n.to_binary_string(), *expected);
    }
}
