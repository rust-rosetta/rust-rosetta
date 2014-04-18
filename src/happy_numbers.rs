// Implements http://rosettacode.org/wiki/Happy_numbers

extern crate collections;
use collections::treemap::TreeSet;

fn digits(m: uint) -> Vec<uint> {
    let mut n = m;
    let mut ds = vec![];
    if n == 0 {
        return vec![0];
    }
    while n > 0 {
        ds.push(n % 10);
        n /= 10;
    }
    ds.reverse();
    ds
}

fn is_happy(c: uint) -> bool {
    let mut d = c;
    let mut past: TreeSet<uint> = TreeSet::new();
    while d != 1 {
        d = digits(d).iter().fold(0, |a, &b| a + b * b);
        if past.contains(&d) {
            return false
        }
        past.insert(d);
    }
    true
}

fn main() {
    let v: Vec<uint> = range(1u, 1000)
        .filter(|x| is_happy(*x))
        .take(8)
        .collect();
    println!("{}", v)
}

#[test]
fn test_digits() {
    assert_eq!(digits(0), vec![0]);
    assert_eq!(digits(1), vec![1]);
    assert_eq!(digits(2), vec![2]);
    assert_eq!(digits(10), vec![1, 0]);
    assert_eq!(digits(11), vec![1, 1]);
    assert_eq!(digits(101), vec![1, 0, 1]);
    assert_eq!(digits(1000), vec![1, 0, 0, 0]);
}

#[test]
fn test_is_happy() {
    let happys = [1u, 7, 10, 13, 19, 23, 28, 31, 1607, 1663];
    let unhappys = [0u, 2, 3, 4, 5, 6, 8, 9, 29, 1662];
    for i in happys.iter() {
        assert_eq!(is_happy(*i), true);
    }

    for i in unhappys.iter() {
        assert_eq!(is_happy(*i), false);
    }
}
