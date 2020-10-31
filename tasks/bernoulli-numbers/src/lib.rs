//! 2.5 implementations here:  naive, optimized, and an iterator using
//! the optimized function. The speeds vary significantly: relative
//! speeds of optimized:iterator:naive implementations is 625:25:1.

extern crate num;

use num::bigint::{BigInt, ToBigInt};
use num::rational::BigRational;
use std::ops::{Mul, Sub};

pub struct Bn {
    pub value: BigRational,
    pub index: i32,
}

pub struct Context {
    bigone_const: BigInt,
    a: Vec<BigRational>,
    index: i32, // Counter for iterator implementation
}

impl Context {
    pub fn new() -> Context {
        let bigone = 1.to_bigint().unwrap();
        let a_vec: Vec<BigRational> = vec![];
        Context {
            bigone_const: bigone,
            a: a_vec,
            index: -1,
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Context {
    type Item = Bn;

    fn next(&mut self) -> Option<Bn> {
        self.index += 1;
        Some(Bn {
            value: bernoulli(self.index as usize, self),
            index: self.index,
        })
    }
}

// Implementation with no reused calculations.
pub fn bernoulli_naive(n: usize, c: &mut Context) -> BigRational {
    for m in 0..n + 1 {
        c.a.push(BigRational::new(
            c.bigone_const.clone(),
            (m + 1).to_bigint().unwrap(),
        ));
        for j in (1..m + 1).rev() {
            c.a[j - 1] = (c.a[j - 1].clone().sub(c.a[j].clone())).mul(BigRational::new(
                j.to_bigint().unwrap(),
                c.bigone_const.clone(),
            ));
        }
    }
    c.a[0].reduced()
}

// Implementation with reused calculations (does not require sequential calls).
pub fn bernoulli(n: usize, c: &mut Context) -> BigRational {
    for i in 0..n + 1 {
        if i >= c.a.len() {
            c.a.push(BigRational::new(
                c.bigone_const.clone(),
                (i + 1).to_bigint().unwrap(),
            ));
            for j in (1..i + 1).rev() {
                c.a[j - 1] = (c.a[j - 1].clone().sub(c.a[j].clone())).mul(BigRational::new(
                    j.to_bigint().unwrap(),
                    c.bigone_const.clone(),
                ));
            }
        }
    }
    c.a[0].reduced()
}

#[cfg(test)]
mod tests {
    use super::{bernoulli, bernoulli_naive, Context};
    use num::rational::BigRational;
    use std::str::FromStr;

    #[test]
    fn test_bernoulli_naive() {
        let mut context = Context::new();
        assert_eq!(
            bernoulli_naive(60, &mut context),
            BigRational::new(
                FromStr::from_str("-1215233140483755572040304994079820246041491").unwrap(),
                FromStr::from_str("56786730").unwrap()
            )
        );
    }

    #[test]
    fn test_bernoulli() {
        let mut context = Context::new();
        assert_eq!(
            bernoulli(60, &mut context),
            BigRational::new(
                FromStr::from_str("-1215233140483755572040304994079820246041491").unwrap(),
                FromStr::from_str("56786730").unwrap()
            )
        );
    }

    #[test]
    fn test_bernoulli_iter() {
        let context = Context::new();
        let res = context.take(60 + 1).collect::<Vec<_>>();
        assert_eq!(
            res.last().unwrap().value,
            BigRational::new(
                FromStr::from_str("-1215233140483755572040304994079820246041491").unwrap(),
                FromStr::from_str("56786730").unwrap()
            )
        );
    }
}
