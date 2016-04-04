// http://rosettacode.org/wiki/Sum_digits_of_an_integer


fn sum(n: usize, base: usize) -> usize {
    let mut total = 0;
    let mut n = n;
    while n != 0 {
        total += n % base;
        n /= base
    }
    total
}

#[test]
fn base_10() {
    assert_eq!(sum(1, 10), 1);
    assert_eq!(sum(1234, 10), 10);
}

#[test]
fn base_16() {
    assert_eq!(sum(0xfe, 16), 29);
    assert_eq!(sum(0xf0e, 16), 29);
}

fn main() {
    println!("{}", sum(1234, 10));
}
