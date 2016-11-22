use std::f32;

fn harmonic_sum<F>(lo: usize, hi: usize, term: F) -> f32
    where F: Fn(f32) -> f32
{
    (lo..hi + 1).fold(0.0, |acc, item| acc + term(item as f32))
}

fn main() {
    println!("{}", harmonic_sum(1, 100, |i| 1.0 / i));
}

#[test]
fn test_harm_sum() {
    let term = |i| 1.0 / i;
    assert!((harmonic_sum(1, 100, &term).abs() - 5.187378) < f32::EPSILON);
    assert!((harmonic_sum(1, 50, &term).abs() - 4.4992056) < f32::EPSILON);
    assert!((harmonic_sum(1, 1000, &term).abs() - 7.4854784) < f32::EPSILON);
    assert!((harmonic_sum(1, 2, &term).abs() - 1.5) < f32::EPSILON);
}
