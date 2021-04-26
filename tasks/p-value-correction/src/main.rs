use std::iter;

#[rustfmt::skip]
const PVALUES:[f64;50] = [
4.533_744e-01, 7.296_024e-01, 9.936_026e-02, 9.079_658e-02, 1.801_962e-01,
8.752_257e-01, 2.922_222e-01, 9.115_421e-01, 4.355_806e-01, 5.324_867e-01,
4.926_798e-01, 5.802_978e-01, 3.485_442e-01, 7.883_130e-01, 2.729_308e-01,
8.502_518e-01, 4.268_138e-01, 6.442_008e-01, 3.030_266e-01, 5.001_555e-02,
3.194_810e-01, 7.892_933e-01, 9.991_834e-01, 1.745_691e-01, 9.037_516e-01,
1.198_578e-01, 3.966_083e-01, 1.403_837e-02, 7.328_671e-01, 6.793_476e-02,
4.040_730e-03, 3.033_349e-04, 1.125_147e-02, 2.375_072e-02, 5.818_542e-04,
3.075_482e-04, 8.251_272e-03, 1.356_534e-03, 1.360_696e-02, 3.764_588e-04,
1.801_145e-05, 2.504_456e-07, 3.310_253e-02, 9.427_839e-03, 8.791_153e-04,
2.177_831e-04, 9.693_054e-04, 6.610_250e-05, 2.900_813e-02, 5.735_490e-03
];

#[derive(Debug)]
enum CorrectionType {
    BenjaminiHochberg,
    BenjaminiYekutieli,
    Bonferroni,
    Hochberg,
    Holm,
    Hommel,
    Sidak,
}

enum SortDirection {
    Increasing,
    Decreasing,
}

/// orders **input** vector by value and multiplies with **multiplier** vector
/// Finally returns the multiplied values in the original order of **input**
fn ordered_multiply(input: &[f64], multiplier: &[f64], direction: &SortDirection) -> Vec<f64> {
    let order_by_value = match direction {
        SortDirection::Increasing => {
            |a: &(f64, usize), b: &(f64, usize)| b.0.partial_cmp(&a.0).unwrap()
        }
        SortDirection::Decreasing => {
            |a: &(f64, usize), b: &(f64, usize)| a.0.partial_cmp(&b.0).unwrap()
        }
    };

    let cmp_minmax = match direction {
        SortDirection::Increasing => |a: f64, b: f64| a.gt(&b),
        SortDirection::Decreasing => |a: f64, b: f64| a.lt(&b),
    };

    // add original order index
    let mut input_indexed = input
        .iter()
        .enumerate()
        .map(|(idx, &p_value)| (p_value, idx))
        .collect::<Vec<_>>();

    // order by value desc/asc
    input_indexed.sort_unstable_by(order_by_value);

    // do the multiplication in place, clamp it at 1.0,
    // keep the original index in place
    for i in 0..input_indexed.len() {
        input_indexed[i] = (
            f64::min(1.0, input_indexed[i].0 * multiplier[i]),
            input_indexed[i].1,
        );
    }

    // make vector strictly monotonous increasing/decreasing in place
    for i in 1..input_indexed.len() {
        if cmp_minmax(input_indexed[i].0, input_indexed[i - 1].0) {
            input_indexed[i] = (input_indexed[i - 1].0, input_indexed[i].1);
        }
    }

    // re-sort back to original order
    input_indexed.sort_unstable_by(|a: &(f64, usize), b: &(f64, usize)| a.1.cmp(&b.1));

    // remove ordering index
    let (resorted, _): (Vec<_>, Vec<_>) = input_indexed.iter().cloned().unzip();
    resorted
}

#[allow(clippy::cast_precision_loss)]
fn hommel(input: &[f64]) -> Vec<f64> {
    // using algorith described:
    // http://stat.wharton.upenn.edu/~steele/Courses/956/ResourceDetails/MultipleComparision/Writght92.pdf

    // add original order index
    let mut input_indexed = input
        .iter()
        .enumerate()
        .map(|(idx, &p_value)| (p_value, idx))
        .collect::<Vec<_>>();

    // order by value asc
    input_indexed
        .sort_unstable_by(|a: &(f64, usize), b: &(f64, usize)| a.0.partial_cmp(&b.0).unwrap());

    let (p_values, order): (Vec<_>, Vec<_>) = input_indexed.iter().cloned().unzip();

    let n = input.len();

    // initial minimal n*p/i values
    // get the smalles of these values
    let min_result = (0..n)
        .map(|i| ((p_values[i] * n as f64) / (i + 1) as f64))
        .fold(1. / 0. /* -inf */, f64::min);

    // // initialize result vector with minimal values
    let mut result = iter::repeat(min_result).take(n).collect::<Vec<_>>();

    for m in (2..n).rev() {
        let cmin: f64;
        let m_as_float = m as f64;
        let mut a = p_values.clone();
        // println!("\nn: {}", m);
        {
            // split p-values into two group
            let (_, second) = p_values.split_at(n - m + 1);

            // calculate minumum of m*p/i for this second group
            cmin = second
                .iter()
                .zip(2..=m)
                .map(|(p, i)| (m_as_float * p) / i as f64)
                .fold(1. / 0. /* inf */, f64::min);
        }

        // replace p values if p<cmin in the second group
        ((n - m + 1)..n).for_each(|i| a[i] = a[i].max(cmin));

        // replace p values if min(cmin, m*p) > p
        (0..=(n - m)).for_each(|i| a[i] = a[i].max(f64::min(cmin, m_as_float * p_values[i])));

        // store in the result vector if any adjusted p is higher than the current one
        (0..n).for_each(|i| result[i] = result[i].max(a[i]));
    }

    // re-sort into the original order
    let mut result = result
        .into_iter()
        .zip(order.into_iter())
        .map(|(p, idx)| (p, idx))
        .collect::<Vec<_>>();
    result.sort_unstable_by(|a: &(f64, usize), b: &(f64, usize)| a.1.cmp(&b.1));
    let (result, _): (Vec<_>, Vec<_>) = result.iter().cloned().unzip();
    result
}
#[allow(clippy::cast_precision_loss)]
fn p_value_correction(p_values: &[f64], ctype: &CorrectionType) -> Vec<f64> {
    let p_vec = p_values.to_vec();
    if p_values.is_empty() {
        return p_vec;
    }

    let fsize = p_values.len() as f64;

    match ctype {
        CorrectionType::BenjaminiHochberg => {
            let multiplier = (0..p_values.len())
                .map(|index| fsize / (fsize - index as f64))
                .collect::<Vec<_>>();

            ordered_multiply(&p_vec, &multiplier, &SortDirection::Increasing)
        }
        CorrectionType::BenjaminiYekutieli => {
            let q: f64 = (1..=p_values.len()).map(|index| 1. / index as f64).sum();
            let multiplier = (0..p_values.len())
                .map(|index| q * fsize / (fsize - index as f64))
                .collect::<Vec<_>>();

            ordered_multiply(&p_vec, &multiplier, &SortDirection::Increasing)
        }
        CorrectionType::Bonferroni => p_vec
            .iter()
            .map(|p| f64::min(p * fsize, 1.0))
            .collect::<Vec<_>>(),
        CorrectionType::Hochberg => {
            let multiplier = (0..p_values.len())
                .map(|index| 1. + index as f64)
                .collect::<Vec<_>>();
            ordered_multiply(&p_vec, &multiplier, &SortDirection::Increasing)
        }
        CorrectionType::Holm => {
            let multiplier = (0..p_values.len())
                .map(|index| fsize - index as f64)
                .collect::<Vec<_>>();

            ordered_multiply(&p_vec, &multiplier, &SortDirection::Decreasing)
        }
        CorrectionType::Sidak => p_vec
            .iter()
            .map(|x| 1. - (1. - x).powf(fsize))
            .collect::<Vec<_>>(),
        CorrectionType::Hommel => hommel(&p_vec),
    }
}

// prints array into a nice table, max 5 floats/row
fn array_to_string(a: &[f64]) -> String {
    a.chunks(5)
        .enumerate()
        .map(|(index, e)| {
            format!(
                "[{:>2}]: {}",
                index * 5,
                e.iter()
                    .map(|x| format!("{:>1.10}", x))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
fn main() {
    let ctypes = [
        CorrectionType::BenjaminiHochberg,
        CorrectionType::BenjaminiYekutieli,
        CorrectionType::Bonferroni,
        CorrectionType::Hochberg,
        CorrectionType::Holm,
        CorrectionType::Sidak,
        CorrectionType::Hommel,
    ];

    for ctype in &ctypes {
        println!("\n{:?}:", ctype);
        println!("{}", array_to_string(&p_value_correction(&PVALUES, ctype)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_p_value_correction_benjamini_hochberg() {
        let result = p_value_correction(
            &[0.001, 0.004, 0.009, 0.007],
            &CorrectionType::BenjaminiHochberg,
        );

        assert_approx_eq!(result[0], 0.004);
        assert_approx_eq!(result[1], 0.008);
        assert_approx_eq!(result[2], 0.009);
        assert_approx_eq!(result[3], 0.009);
    }

    #[test]
    fn test_p_value_correction_benjamini_yekutieli() {
        let result = p_value_correction(
            &[0.001, 0.004, 0.009, 0.007],
            &CorrectionType::BenjaminiYekutieli,
        );

        assert_approx_eq!(result[0], 0.008333333);
        assert_approx_eq!(result[1], 0.016666667);
        assert_approx_eq!(result[2], 0.018750000);
        assert_approx_eq!(result[3], 0.018750000);
    }

    #[test]
    fn test_p_value_correction_bonferroni() {
        let result = p_value_correction(&[0.001, 0.004, 0.009, 0.007], &CorrectionType::Bonferroni);

        assert_approx_eq!(result[0], 0.004);
        assert_approx_eq!(result[1], 0.016);
        assert_approx_eq!(result[2], 0.036);
        assert_approx_eq!(result[3], 0.028);
    }

    #[test]
    fn test_p_value_correction_hochberg() {
        let result = p_value_correction(&[0.001, 0.004, 0.009, 0.007], &CorrectionType::Hochberg);

        assert_approx_eq!(result[0], 0.004);
        assert_approx_eq!(result[1], 0.009);
        assert_approx_eq!(result[2], 0.009);
        assert_approx_eq!(result[3], 0.009);
    }

    #[test]
    fn test_p_value_correction_holm() {
        let result = p_value_correction(&[0.001, 0.004, 0.009, 0.007], &CorrectionType::Holm);

        assert_approx_eq!(result[0], 0.004);
        assert_approx_eq!(result[1], 0.012);
        assert_approx_eq!(result[2], 0.014);
        assert_approx_eq!(result[3], 0.014);
    }

    #[test]
    fn test_p_value_correction_sidak() {
        let result = p_value_correction(&[0.001, 0.004, 0.009, 0.007], &CorrectionType::Sidak);

        assert_approx_eq!(result[0], 0.003994003998999962);
        assert_approx_eq!(result[1], 0.015904255744000007);
        assert_approx_eq!(result[2], 0.03551690943899999);
        assert_approx_eq!(result[3], 0.02770736959900011);
    }
    #[test]
    fn test_p_value_correction_hommel() {
        let result = p_value_correction(&[0.001, 0.004, 0.009, 0.007], &CorrectionType::Hommel);

        assert_approx_eq!(result[0], 0.004);
        assert_approx_eq!(result[1], 0.009);
        assert_approx_eq!(result[2], 0.009);
        assert_approx_eq!(result[3], 0.009);
    }
}
