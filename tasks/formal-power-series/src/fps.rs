use std::fmt;
use std::fmt::Write;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use num::rational::BigRational;
use num::{BigInt, One, Signed, Zero};

const MAX_DISPLAYED_TERMS: usize = 24;

lazy_static! {
    static ref ZERO: FormalPowerSeries =
        FormalPowerSeries::new(Arc::new(FormalPowerSeries::zero_func));
    static ref ONE: FormalPowerSeries =
        FormalPowerSeries::new(Arc::new(FormalPowerSeries::one_func));
}

type Coef = Arc<dyn Fn(usize) -> BigRational + Send + Sync>;
type Cache = Arc<Mutex<Vec<BigRational>>>;

#[derive(Clone)]
pub struct FormalPowerSeries {
    // nobody else seems to be using bigints for the term index parameter, so usize it is
    coef: Coef,
}

impl FormalPowerSeries {
    /// Helper for creating the global zero
    fn zero_func(_: usize) -> BigRational {
        BigRational::zero()
    }

    /// Helper for creating the global one
    fn one_func(n: usize) -> BigRational {
        if n == 0 {
            BigRational::one()
        } else {
            BigRational::zero()
        }
    }

    /// Returns the global additive identity
    pub fn zero() -> &'static Self {
        &*ZERO
    }

    /// Returns the global multiplicative identity
    pub fn one() -> &'static Self {
        &*ONE
    }

    /// Construct a new formal power series
    pub fn new(coef: Coef) -> Self {
        FormalPowerSeries { coef }
    }

    /// Returns a string of Unicode superscript characters corresponding to the base-10
    /// representation of exp. Example: `12` becomes `¹²`.
    fn exp_string(mut exp: usize) -> String {
        const EXPS: [char; 10] = [
            '\u{2070}', '\u{00B9}', '\u{00B2}', '\u{00B3}', '\u{2074}', '\u{2075}', '\u{2076}',
            '\u{2077}', '\u{2078}', '\u{2079}',
        ];
        if exp == 1 {
            return String::new();
        } else if exp == 0 {
            return String::from('\u{2070}');
        }
        let mut digits = Vec::new();
        let num_digits = (exp.log10() + 1) as usize; // obviously safe
        digits.resize(num_digits, '\0');
        for i in 0..num_digits {
            let digit = (exp % 10) as usize; // obviously safe
            digits[num_digits - (i + 1)] = EXPS[digit];
            exp /= 10;
        }
        digits.into_iter().collect()
    }

    /// Returns the xⁿ term of the series
    pub fn nth(&self, n: usize) -> BigRational {
        (self.coef)(n)
    }

    /// Helper function for computing the inverse
    fn inv_coef(&self, n: usize, cache: Cache) -> BigRational {
        if n == 0 {
            BigRational::one() / self.nth(0)
        } else {
            let cache_vec = cache.lock().unwrap();
            let len = cache_vec.len();
            if len > n {
                cache_vec[n].clone()
            } else {
                drop(cache_vec); // since we need to pass the cache to the recursive calls
                let mut prod_others = BigRational::zero();
                for i in 0..n {
                    prod_others -= self.nth(n - i) * self.inv_coef(i, cache.clone())
                }
                let result = prod_others / self.nth(0);
                let mut cache_vec = cache.lock().unwrap();
                cache_vec.resize(n + 1, BigRational::zero());
                cache_vec[n] = result;
                cache_vec[n].clone()
            }
        }
    }

    /// Returns the inverse, which is the series _s_ such that _self_ * _s_ = 1. Only valid for
    /// series with nonzero constant term.
    pub fn inverse(&self) -> Self {
        if self.nth(0) == BigRational::zero() {
            panic!("cannot take the inverse of a series with constant term zero");
        }
        let copy = self.clone();
        let inv_coef = Arc::new(move |n| {
            let cache = Arc::new(Mutex::new(Vec::with_capacity(MAX_DISPLAYED_TERMS)));
            copy.inv_coef(n, cache)
        });
        FormalPowerSeries::new(inv_coef)
    }

    /// Returns the first derivative of the series with respect to x
    pub fn derivative(&self) -> Self {
        let copy = self.clone();
        let deriv_coef =
            Arc::new(move |n| copy.nth(n + 1) * BigRational::from(BigInt::from(n + 1)));
        FormalPowerSeries::new(deriv_coef)
    }

    /// Returns the definite integral of the series with lower limit 0 and upper limit x
    pub fn integral(&self) -> Self {
        let copy = self.clone();
        let int_coef = Arc::new(move |n| {
            if n > 0 {
                copy.nth(n - 1) * BigRational::new(BigInt::one(), BigInt::from(n))
            } else {
                BigRational::zero()
            }
        });
        FormalPowerSeries::new(int_coef)
    }

    pub fn iter(&self) -> Iter {
        Iter {
            series: self,
            cur_term: 0,
        }
    }

    pub fn iter_nonzero(&self) -> IterNonzero {
        IterNonzero {
            series: self,
            cur_term: 0,
        }
    }
}

// Here and in the other ops the closures are shared since FormalPowerSeries is a persistent type
impl Add for FormalPowerSeries {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let sum_coef = Arc::new(move |index| self.nth(index) + rhs.nth(index));
        FormalPowerSeries::new(sum_coef)
    }
}

impl Sub for FormalPowerSeries {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff_coef = Arc::new(move |index| self.nth(index) - rhs.nth(index));
        FormalPowerSeries::new(diff_coef)
    }
}

impl Mul for FormalPowerSeries {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let prod_coef = Arc::new(move |index| {
            let mut prod = BigRational::zero();
            for i in 0..=index {
                prod += self.nth(i) * rhs.nth(index - i);
            }
            prod
        });
        FormalPowerSeries::new(prod_coef)
    }
}

impl Div for FormalPowerSeries {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

impl Neg for FormalPowerSeries {
    type Output = Self;

    fn neg(self) -> Self::Output {
        FormalPowerSeries::new(Arc::new(move |n| -self.nth(n)))
    }
}

impl Add for &FormalPowerSeries {
    type Output = FormalPowerSeries;

    fn add(self, rhs: Self) -> Self::Output {
        let lcpy = self.clone();
        let rcpy = rhs.clone();
        let sum_coef = Arc::new(move |n| lcpy.nth(n) + rcpy.nth(n));
        FormalPowerSeries::new(sum_coef)
    }
}

impl Sub for &FormalPowerSeries {
    type Output = FormalPowerSeries;

    fn sub(self, rhs: Self) -> Self::Output {
        let lcpy = self.clone();
        let rcpy = rhs.clone();
        let diff_coef = Arc::new(move |n| lcpy.nth(n) - rcpy.nth(n));
        FormalPowerSeries::new(diff_coef)
    }
}

impl Mul for &FormalPowerSeries {
    type Output = FormalPowerSeries;

    fn mul(self, rhs: Self) -> Self::Output {
        let lcpy = self.clone();
        let rcpy = rhs.clone();
        let prod_coef = Arc::new(move |n| {
            let mut prod = BigRational::zero();
            for i in 0..=n {
                prod += lcpy.nth(i) * rcpy.nth(n - i);
            }
            prod
        });
        FormalPowerSeries::new(prod_coef)
    }
}

impl Div for &FormalPowerSeries {
    type Output = FormalPowerSeries;

    fn div(self, rhs: Self) -> Self::Output {
        self * &rhs.inverse()
    }
}

impl Neg for &FormalPowerSeries {
    type Output = FormalPowerSeries;

    fn neg(self) -> Self::Output {
        let copy = self.clone();
        FormalPowerSeries::new(Arc::new(move |n| -copy.nth(n)))
    }
}

impl fmt::Display for FormalPowerSeries {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for term in self.iter().take(MAX_DISPLAYED_TERMS) {
            if term.coef == BigRational::zero() {
                continue;
            }
            let exp_string = FormalPowerSeries::exp_string(term.exp);
            const EXP_ZERO: &str = "\u{2070}";
            if exp_string == EXP_ZERO {
                write!(s, "{}", term.coef)?;
            } else if s.is_empty() {
                if term.coef == BigRational::one() {
                    write!(s, "x{}", exp_string)?;
                } else if term.coef.abs() == BigRational::one() {
                    write!(s, "-x{}", exp_string)?;
                } else {
                    write!(s, "{} x{}", term.coef, exp_string)?;
                }
            } else {
                let sign = if term.coef > BigRational::zero() {
                    '+'
                } else {
                    '-'
                };
                let coef = term.coef.abs();
                if coef == BigRational::one() {
                    write!(s, " {} x{}", sign, exp_string)?;
                } else {
                    write!(s, " {} {} x{}", sign, term.coef.abs(), exp_string)?;
                }
            }
        }
        if s.is_empty() {
            write!(f, "0")?;
        } else {
            write!(f, "{}", s)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Term {
    pub exp: usize,
    pub coef: BigRational,
}

pub struct Iter<'a> {
    series: &'a FormalPowerSeries,
    cur_term: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Term;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_term == usize::MAX {
            None
        } else {
            let coef = self.series.nth(self.cur_term);
            let exp = self.cur_term;
            self.cur_term += 1;
            Some(Term { exp, coef })
        }
    }
}

pub struct IterNonzero<'a> {
    series: &'a FormalPowerSeries,
    cur_term: usize,
}

impl<'a> Iterator for IterNonzero<'a> {
    type Item = Term;

    fn next(&mut self) -> Option<Self::Item> {
        for exp in (self.cur_term + 1).. {
            let coef = self.series.nth(exp);
            if coef != BigRational::zero() {
                self.cur_term = exp;
                return Some(Term { exp, coef });
            }
        }
        unreachable!()
    }
}
