fn median(samples: &[f64]) -> f64 {
    let mut xs = samples.iter().cloned().collect::<Vec<_>>();
    xs.sort_by(|x, y| x.partial_cmp(y).unwrap());

    let n = xs.len();
    if n % 2 == 0 {
        (xs[n / 2] + xs[(n / 2) + 1]) / 2.0
    } else {
        xs[n / 2]
    }
}

fn main() {
    let nums = vec![2., 3., 5., 0., 9., 82., 353., 32., 12.];
    println!("{:?}", median(&nums))
}

#[cfg(test)]
mod tests {
    use std::f64;

    #[test]
    fn median() {
        let nums = vec![2., 3., 5., 0., 9., 82., 353., 32., 12.];
        assert!((super::median(&nums) - 9_f64).abs() < f64::EPSILON);
    }
}
