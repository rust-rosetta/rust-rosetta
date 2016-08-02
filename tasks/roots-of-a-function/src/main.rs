extern crate num;

use num::Float;

/// Note: We cannot use `range_step` here because Floats don't implement
/// the `CheckedAdd` trait.
fn find_roots<T, F>(f: F, start: T, stop: T, step: T, epsilon: T) -> Vec<T>
    where T: Copy + PartialOrd + Float,
          F: Fn(T) -> T
{
    let mut ret = vec![];
    let mut current = start;
    while current < stop {
        if f(current).abs() < epsilon {
            ret.push(current);
        }
        current = current + step;
    }
    ret
}

#[test]
fn test_find_roots() {
    let roots = find_roots(|x: f64| x * x * x - 3.0 * x * x + 2.0 * x,
                           -1.0,
                           3.0,
                           0.0001,
                           0.00000001);
    let expected = [0.0f64, 1.0, 2.0];

    for (&a, &b) in roots.iter().zip(expected.iter()) {
        assert!((a - b).abs() < 0.0001);
    }
}

fn main() {
    let roots = find_roots(|x: f64| x * x * x - 3.0 * x * x + 2.0 * x,
                           -1.0,
                           3.0,
                           0.0001,
                           0.00000001);

    println!("roots of f(x) = x^3 - 3x^2 + 2x are: {:?}", roots);
}
