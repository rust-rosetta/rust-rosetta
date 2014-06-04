// Implements http://rosettacode.org/wiki/Binary_digits
use std::vec::Vec;
use std::string::String;

#[cfg(not(test))]
fn main() {
    let bins=binaries(16u);
    for s in bins.iter() {
        println!("{}", s);
    }
}

fn binaries(n:uint) -> Vec<String> {
    range(0, n).map(|i| format!("{:t}", i)).collect()
}

#[test]
fn test_digits() {
    let bins=binaries(16u);
    assert_eq!(bins, vec![String::from_str("0")
                        ,String::from_str("1")
                        ,String::from_str("10")
                        ,String::from_str("11")
                        ,String::from_str("100")
                        ,String::from_str("101")
                        ,String::from_str("110")
                        ,String::from_str("111")
                        ,String::from_str("1000")
                        ,String::from_str("1001")
                        ,String::from_str("1010")
                        ,String::from_str("1011")
                        ,String::from_str("1100")
                        ,String::from_str("1101")
                        ,String::from_str("1110")
                        ,String::from_str("1111")]);
}
