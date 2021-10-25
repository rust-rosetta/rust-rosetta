#![feature(int_log)]
#![feature(once_cell)]

mod fps;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use lazy_static::lazy_static;
    use num::{BigInt, BigRational, One, Zero};

    use super::fps::*;

    lazy_static! {
        static ref N_PLUS_ONE: FormalPowerSeries =
            FormalPowerSeries::new(Arc::new(|n| { BigRational::from(BigInt::from(n + 1)) }));
        static ref TWO_TIMES_N: FormalPowerSeries =
            FormalPowerSeries::new(Arc::new(|n| { BigRational::from(BigInt::from(2 * n)) }));
        static ref ONE_MINUS_X_SQ: FormalPowerSeries = FormalPowerSeries::new(Arc::new(|n| {
            if n == 0 {
                BigRational::one()
            } else if n == 1 {
                BigRational::from(BigInt::from(-2))
            } else if n == 2 {
                BigRational::one()
            } else {
                BigRational::zero()
            }
        }));
        static ref SIN: FormalPowerSeries = FormalPowerSeries::new(Arc::new(move |n| {
            if n == 0 {
                BigRational::zero()
            } else {
                COS.integral().nth(n)
            }
        }));
        static ref COS: FormalPowerSeries = FormalPowerSeries::new(Arc::new(move |n| {
            if n == 0 {
                BigRational::one()
            } else {
                (FormalPowerSeries::one() - &SIN.integral()).nth(n)
            }
        }));
    }

    #[test]
    #[cfg(feature = "five_terms")]
    fn create_and_print() {
        let fps = &*N_PLUS_ONE;
        assert_eq!(
            fps.to_string(),
            "1 + 2 x + 3 x\u{00B2} + 4 x\u{00B3} + 5 x\u{2074}"
        )
    }

    #[test]
    fn zero_one() {
        let zero = FormalPowerSeries::zero();
        let one = FormalPowerSeries::one();
        assert_eq!(zero.to_string(), "0");
        assert_eq!(one.to_string(), "1");
    }

    #[test]
    #[cfg(feature = "five_terms")]
    fn arithmetic() {
        let fps1 = &*N_PLUS_ONE;
        let fps2 = &*TWO_TIMES_N;
        let poly = &*ONE_MINUS_X_SQ;
        assert_eq!(
            (fps1 + fps2).to_string(),
            "1 + 4 x + 7 x\u{00B2} + 10 x\u{00B3} + 13 x\u{2074}"
        );
        assert_eq!(
            (fps1 - fps2).to_string(),
            "1 - x\u{00B2} - 2 x\u{00B3} - 3 x\u{2074}"
        );
        assert_eq!(
            (fps1 * fps2).to_string(),
            "2 x + 8 x\u{00B2} + 20 x\u{00B3} + 40 x\u{2074}"
        );
        assert_eq!(
            (poly / fps1).to_string(),
            "1 - 4 x + 6 x\u{00B2} - 4 x\u{00B3} + x\u{2074}"
        );
    }

    #[test]
    fn inverse() {
        let fps1 = &*N_PLUS_ONE;
        let fps2 = &*TWO_TIMES_N;
        let poly = &*ONE_MINUS_X_SQ;
        assert_eq!(fps1.inverse().to_string(), poly.to_string());
        assert_eq!(
            (fps1 * poly).to_string(),
            FormalPowerSeries::one().to_string()
        );
        assert_eq!((fps2 / fps1).to_string(), "2 x")
    }

    #[test]
    #[should_panic]
    fn invert_zero_constant_term() {
        let zct = &*TWO_TIMES_N;
        zct.inverse();
    }

    #[test]
    fn calculus() {
        let fps1 = FormalPowerSeries::one() + &(*N_PLUS_ONE).integral();
        let fps2 = FormalPowerSeries::new(Arc::new(move |_| BigRational::one()));
        assert_eq!(fps1.to_string(), fps2.to_string());
        assert_eq!(fps2.derivative().to_string(), *N_PLUS_ONE.to_string());
    }

    #[test]
    fn sin_cos() {
        let sin_series = &*SIN;
        let cos_series = &*COS;

        println!("sin(x) = {}", sin_series);
        println!("cos(x) = {}", cos_series);

        // Definitions
        assert_eq!(sin_series.to_string(), cos_series.integral().to_string());
        assert_eq!(
            cos_series.to_string(),
            (FormalPowerSeries::one() - &sin_series.integral()).to_string()
        );

        // Pythagorean identity
        assert_eq!(
            (sin_series * sin_series + cos_series * cos_series).to_string(),
            FormalPowerSeries::one().to_string()
        );
    }

    #[test]
    #[cfg(feature = "five_terms")]
    fn sin_cos_explicit() {
        let sin_series = &*SIN;
        let cos_series = &*COS;
        assert_eq!(sin_series.to_string(), "x - 1/6 x\u{00B3}");
        assert_eq!(cos_series.to_string(), "1 - 1/2 x\u{00B2} + 1/24 x\u{2074}");
    }
}
