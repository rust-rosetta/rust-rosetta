use ndarray::prelude::*;
use ndarray::Data;
use ndarray::Zip;

use num::traits::Zero;
use std::ops::Mul;

// works only for 2D arrays
fn kronecker_product<S, T>(a: &ArrayBase<S, Ix2>, b: &ArrayBase<S, Ix2>) -> Array<T, Ix2>
where
    T: Copy + Zero + Mul<Output = T>,
    S: Data<Elem = T>,
{
    // create an empty array to hold the results
    let mut res: Array2<T> = Array2::zeros(Dim([
        a.raw_dim()[0] * b.raw_dim()[0],
        a.raw_dim()[1] * b.raw_dim()[1],
    ]));

    // break down the result array into chunks
    // according to the second array
    let res_chunks = res.exact_chunks_mut((b.raw_dim()[0], b.raw_dim()[1]));

    // fill in the product of the two cells
    Zip::from(res_chunks).and(a).apply(|res_chunk, &a_elem| {
        Zip::from(b).apply_assign_into(res_chunk, |&b_elem| a_elem * b_elem)
    });

    res
}

fn main() {
    let a = array![[1, 2], [3, 4]];
    let b = array![[0, 5], [6, 7]];

    let res = kronecker_product(&a, &b);
    println!("RESULT:\n{}", res);

    let a = array![[0, 1, 0], [1, 1, 1], [0, 1, 0]];
    let b = array![[1, 1, 1, 1], [1, 0, 0, 1], [1, 1, 1, 1]];

    let res = kronecker_product(&a, &b);
    println!("RESULT:\n{}", res);
}

#[cfg(test)]
mod tests {
    use super::kronecker_product;
    use ::ndarray::prelude::*;

    #[test]
    fn test_kronecker_product() {
        let a = array![[1, 2], [3, 4]];
        let b = array![[0, 5], [6, 7]];

        assert_eq!(
            kronecker_product(&a, &b),
            array![
                [0, 5, 0, 10],
                [6, 7, 12, 14],
                [0, 15, 0, 20],
                [18, 21, 24, 28]
            ]
        );

        let a = array![[1, 2], [3, 4]];
        let b = array![[0, 5, 1], [6, 7, 2]];

        assert_eq!(
            kronecker_product(&a, &b),
            array![
                [0, 5, 1, 0, 10, 2],
                [6, 7, 2, 12, 14, 4],
                [0, 15, 3, 0, 20, 4],
                [18, 21, 6, 24, 28, 8]
            ]
        );
    }
}
