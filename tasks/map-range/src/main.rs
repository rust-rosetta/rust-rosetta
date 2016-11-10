use std::f64;

fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

fn main() {
    let input: Vec<f64> = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let result = input.into_iter()
        .map(|x| map_range((0.0, 10.0), (-1.0, 0.0), x))
        .collect::<Vec<f64>>();
    print!("{:?}", result);
}

#[test]
fn test_basic() {
    assert!((map_range((1.0, 5.0), (10.0, 50.0), 2.0) - 20.0) < f64::EPSILON);
}
