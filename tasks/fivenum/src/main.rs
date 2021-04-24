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
            0.140_828_34,
            0.097_487_90,
            1.731_315_07,
            0.876_360_09,
            -1.950_595_94,
            0.734_385_55,
            -0.030_357_26,
            1.466_759_70,
            -0.746_213_49,
            -0.725_887_72,
            0.639_051_60,
            0.615_015_27,
            -0.989_837_80,
            -1.004_478_74,
            -0.627_594_69,
            0.662_061_63,
            1.043_120_09,
            -0.103_053_85,
            0.757_756_34,
            0.325_665_78,
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
