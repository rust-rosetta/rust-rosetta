use nalgebra::DMatrix;
/// For a given a set of data vectors in the following format y = {y₁, y₂, ... yn}
/// and X = {xi₁, xi₂, ... xin}, where i 𝜖 1..k
///
/// computes the vector β = {β₁, β₂, ...βk} using ordinary least squares regression
fn get_coefficients(x: &DMatrix<f64>, y: &DMatrix<f64>) -> DMatrix<f64> {
    (x.transpose() * x).try_inverse().unwrap() * x.transpose() * y
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
fn main() {
    let x = vec![
        1.47_f64, 1.50, 1.52, 1.55, 1.57, 1.60, 1.63, 1.65, 1.68, 1.70, 1.73, 1.75, 1.78, 1.80,
        1.83,
    ];

    let y = vec![
        52.21_f64, 53.12, 54.48, 55.84, 57.20, 58.57, 59.93, 61.29, 63.11, 64.47, 66.28, 68.10,
        69.92, 72.19, 74.46,
    ];

    // generate 1, x, and x^2
    let x = DMatrix::from_fn(x.len(), 3, |row, col| x[row].powi(col as i32));
    let y = DMatrix::from_vec(y.len(), 1, y);
    println!(
        "β parameters using least squares regression :{}",
        get_coefficients(&x, &y)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_get_coefficients() {
        let x = DMatrix::from_vec(5, 1, vec![2_f64, 1., 3., 4., 5.]);
        let y = DMatrix::from_vec(5, 1, vec![1_f64, 2., 3., 4., 5.]);
        let result = get_coefficients(&x, &y);
        assert_approx_eq!(result[(0)], 0.9818181818181818);

        let x = DMatrix::from_vec(3, 2, vec![1_f64, 2., 1., 1., 1., 2.]);
        let y = DMatrix::from_vec(3, 1, vec![3_f64, 4., 5.]);
        let result = get_coefficients(&x, &y);
        assert_approx_eq!(result[(0)], 1.);
        assert_approx_eq!(result[(1)], 2.);
    }
}
