fn f(n: usize) -> usize {
    match n {
        0 => 1,
        _ => n - m(f(n - 1)),
    }
}

fn m(n: usize) -> usize {
    match n {
        0 => 0,
        _ => n - f(m(n - 1)),
    }
}

fn main() {
    for i in (0..20).map(f) {
        print!("{} ", i);
    }
    println!("");

    for i in (0..20).map(m) {
        print!("{} ", i);
    }
    println!("");
}

#[test]
fn test_mutual_recursion() {

    let f_expected = [1, 1, 2, 2, 3, 3, 4, 5, 5, 6, 6, 7, 8, 8, 9, 9, 10, 11, 11, 12];
    let m_expected = [0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 6, 7, 7, 8, 9, 9, 10, 11, 11, 12];
    let f_m_zipped = f_expected.iter().zip(m_expected.iter());
    for (i, (f_expect, m_expect)) in f_m_zipped.enumerate() {
        assert!(f(i) == *f_expect);
        assert!(m(i) == *m_expect);
    }

}
