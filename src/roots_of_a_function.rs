// http://rosettacode.org/wiki/Roots_of_a_function
use std::num::Zero;

// Note: We cannot use `range_step` + `filter` + `collect` here because Floats
// don't implement `CheckedAdd` trait.
fn find_roots<T: PartialOrd + Signed + Copy> (f: |T| -> T, start: T, end: T,
                                              step: T, epsilon: T) -> Vec<T> {
    let mut ret = vec![];
    let mut current = start;
    while current < end {
        if f(current).abs() < epsilon {
            ret.push(current)
        }
        current = current + step;
    }
    ret
}

#[test]
fn test_find_roots() {
    let roots = find_roots(|x: f64| x*x*x - 3.0*x*x + 2.0*x,
                           -1.0, 3.0, 0.0001, 0.00000001);
    assert_eq!(roots, vec![0.0, 1.0, 2.0])
}

fn main() {
    let roots = find_roots(|x: f64| x*x*x - 3.0*x*x + 2.0*x,
                           -1.0, 3.0, 0.0001, 0.00000001);

    println!("roots of f(x) = x^3 - 3x^2 + 2x are: {}", roots);
}
