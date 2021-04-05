#[derive(Debug)]
struct FiveNum {
    minimum: f64,
    lower_quartile: f64,
    median: f64,
    upper_quartile: f64,
    maximum: f64,
}

fn median(samples: &[f64]) -> f64 {
    // input is already sorted
    let n = samples.len();
    let m = n / 2;
    if n % 2 == 0 {
        (samples[m] + samples[m - 1]) / 2.0
    } else {
        samples[m]
    }
}

fn fivenum(samples: &[f64]) -> FiveNum {
    let mut xs = samples.to_vec();
    xs.sort_by(|x, y| x.partial_cmp(y).unwrap());

    let m = xs.len() / 2;

    FiveNum {
        minimum: xs[0],
        lower_quartile: median(&xs[0..(m + (xs.len() % 2))]),
        median: median(&xs),
        upper_quartile: median(&xs[m..]),
        maximum: xs[xs.len() - 1],
    }
}
fn main() {
    let inputs = vec![
        vec![15., 6., 42., 41., 7., 36., 49., 40., 39., 47., 43.],
        vec![36., 40., 7., 39., 41., 15.],
        vec![
            0.14082834,
            0.09748790,
            1.73131507,
            0.87636009,
            -1.95059594,
            0.73438555,
            -0.03035726,
            1.46675970,
            -0.74621349,
            -0.72588772,
            0.63905160,
            0.61501527,
            -0.98983780,
            -1.00447874,
            -0.62759469,
            0.66206163,
            1.04312009,
            -0.10305385,
            0.75775634,
            0.32566578,
        ],
    ];

    for input in inputs {
        let result = fivenum(&input);
        println!("Fivenum",);
        println!("  Minumum: {}", result.minimum);
        println!("  Lower quartile: {}", result.lower_quartile);
        println!("  Median: {}", result.median);
        println!("  Upper quartile: {}", result.upper_quartile);
        println!("  Maximum: {}", result.maximum);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_fivenum() {
        let input = vec![1., 2.];
        let result = fivenum(&input);

        assert_approx_eq!(result.minimum, 1.);
        assert_approx_eq!(result.lower_quartile, 1.);
        assert_approx_eq!(result.median, 1.5);
        assert_approx_eq!(result.upper_quartile, 2.);
        assert_approx_eq!(result.maximum, 2.);

        let input = vec![36., 40., 7., 39., 41., 15.];
        let result = fivenum(&input);

        assert_approx_eq!(result.minimum, 7.);
        assert_approx_eq!(result.lower_quartile, 15.);
        assert_approx_eq!(result.median, 37.5);
        assert_approx_eq!(result.upper_quartile, 40.);
        assert_approx_eq!(result.maximum, 41.);
    }
}
