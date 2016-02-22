// http://rosettacode.org/wiki/Averages/Median

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
    let mut nums = vec![2., 3., 5., 0., 9., 82., 353., 32., 12.];
    println!("{:?}", median(&mut nums))
}

mod tests {
    #[test]
    fn median() {
        let mut nums = vec![2., 3., 5., 0., 9., 82., 353., 32., 12.];
        assert_eq!(super::median(&mut nums), 9_f64);
    }
}
