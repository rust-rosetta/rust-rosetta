use std::iter;

// Calculating a continued fraction is quite easy with iterators, however
// writing a proper iterator adapter is less so. We settle for a macro which
// for most purposes works well enough.
//
// One limitation with this iterator based approach is that we cannot reverse
// input iterators since they are not usually DoubleEnded. To circumvent this
// we can collect the elements and then reverse them, however this isn't ideal
// as we now have to store elements equal to the number of iterations.
//
// Another is that iterators cannot be resused once consumed, so it is often
// required to make many clones of iterators.
macro_rules! continued_fraction {
    ($a:expr, $b:expr ; $iterations:expr) => (
        ($a).zip($b)
            .take($iterations)
            .collect::<Vec<_>>().iter()
            .rev()
            .fold(0 as f64, |acc: f64, &(x, y)| {
                x as f64 + (y as f64 / acc)
            })
    );

    ($a:expr, $b:expr) => (continued_fraction!($a, $b ; 1000));
}

fn main() {
    // Sqrt(2)
    let sqrt2_a = (1..2).chain(iter::repeat(2));
    let sqrt2_b = iter::repeat(1);
    println!("{}", continued_fraction!(sqrt2_a, sqrt2_b));


    // Napier's Constant
    let napier_a = (2..3).chain(1..);
    let napier_b = (1..2).chain(1..);
    println!("{}", continued_fraction!(napier_a, napier_b));


    // Pi
    let pi_a = (3..4).chain(iter::repeat(6));
    let pi_b = (1i64..).map(|x| (2 * x - 1).pow(2));
    println!("{}", continued_fraction!(pi_a, pi_b));
}

#[cfg_attr(feature="clippy", allow(float_cmp, approx_constant))]
#[cfg(test)]
mod tests {
    use std::iter;

    #[test]
    fn test_sqrt2() {
        let sqrt2_a = (1..2).chain(iter::repeat(2));
        let sqrt2_b = iter::repeat(1);

        // Note that we must clone the iterator here if we want to reuse
        assert_eq!(continued_fraction!(sqrt2_a.clone(), sqrt2_b.clone() ; 10),
                   1.4142131979695431f64);

        assert_eq!(continued_fraction!(sqrt2_a.clone(), sqrt2_b.clone()),
                   continued_fraction!(sqrt2_a.clone(), sqrt2_b.clone() ; 1000));

        assert_eq!(continued_fraction!(sqrt2_a, sqrt2_b ; 73),
                   1.4142135623730951f64);
    }
}
