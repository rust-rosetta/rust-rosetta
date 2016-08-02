//! 2.5 implementations here:  naive, optimized, and an iterator using
//! the optimized function. The speeds vary significantly: relative
//! speeds of optimized:iterator:naive implementations is 625:25:1.

#![feature(test)]

extern crate num;
extern crate test;

use num::bigint::{BigInt, ToBigInt};
use num::rational::BigRational;
use std::cmp::max;
use std::env;
use std::ops::{Mul, Sub};
use std::process;

struct Bn {
    value: BigRational,
    index: i32,
}

struct Context {
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

fn help() {
    println!("Usage: bernoulli_numbers <up_to>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut up_to: usize = 60;

    match args.len() {
        1 => {}
        2 => {
            up_to = args[1].parse::<usize>().unwrap();
        }
        _ => {
            help();
            process::exit(0);
        }
    }

    let context = Context::new();
    // Collect the solutions by using the Context iterator
    // (this is not as fast as calling the optimized function directly).
    let res = context.take(up_to + 1).collect::<Vec<_>>();
    let width = res.iter().fold(0, |a, r| max(a, r.value.numer().to_string().len()));

    for r in res.iter().filter(|r| r.index % 2 == 0) {
        println!("B({:>2}) = {:>2$} / {denom}",
                 r.index,
                 r.value.numer(),
                 width,
                 denom = r.value.denom());
    }
}

// Implementation with no reused calculations.
fn _bernoulli_naive(n: usize, c: &mut Context) -> BigRational {
    for m in 0..n + 1 {
        c.a.push(BigRational::new(c.bigone_const.clone(), (m + 1).to_bigint().unwrap()));
        for j in (1..m + 1).rev() {
            c.a[j - 1] = (c.a[j - 1].clone().sub(c.a[j].clone()))
                .mul(BigRational::new(j.to_bigint().unwrap(), c.bigone_const.clone()));
        }
    }
    c.a[0].reduced()
}

// Implementation with reused calculations (does not require sequential calls).
fn bernoulli(n: usize, c: &mut Context) -> BigRational {
    for i in 0..n + 1 {
        if i >= c.a.len() {
            c.a.push(BigRational::new(c.bigone_const.clone(), (i + 1).to_bigint().unwrap()));
            for j in (1..i + 1).rev() {
                c.a[j - 1] = (c.a[j - 1].clone().sub(c.a[j].clone()))
                    .mul(BigRational::new(j.to_bigint().unwrap(), c.bigone_const.clone()));
            }
        }
    }
    c.a[0].reduced()
}


#[cfg(test)]
mod tests {
    use super::{Bn, Context, bernoulli, _bernoulli_naive};
    use num::rational::BigRational;
    use std::str::FromStr;
    use test::Bencher;

    #[test]
    fn test_bernoulli_naive() {
        let mut context = Context::new();
        assert_eq!(_bernoulli_naive(60, &mut context), BigRational::new(
                FromStr::from_str("-1215233140483755572040304994079820246041491").unwrap(),
                FromStr::from_str("56786730").unwrap()
            )
        );
    }

    #[test]
    fn test_bernoulli() {
        let mut context = Context::new();
        assert_eq!(bernoulli(60, &mut context), BigRational::new(
                FromStr::from_str("-1215233140483755572040304994079820246041491").unwrap(),
                FromStr::from_str("56786730").unwrap()
            )
        );
    }

    #[test]
    fn test_bernoulli_iter() {
        let context = Context::new();
        let res = context.take(60 + 1).collect::<Vec<_>>();
        assert_eq!(res.last().unwrap().value, BigRational::new(
                FromStr::from_str("-1215233140483755572040304994079820246041491").unwrap(),
                FromStr::from_str("56786730").unwrap()
            )
        );
    }

    #[bench]
    fn bench_bernoulli_naive(b: &mut Bencher) {
        let mut context = Context::new();
        b.iter(|| {
            let mut res: Vec<Bn> = vec![];
            for n in 0..30 + 1 {
                let b = _bernoulli_naive(n, &mut context);
                res.push(Bn {
                    value: b.clone(),
                    index: n as i32,
                });
            }
        });
    }

    #[bench]
    fn bench_bernoulli(b: &mut Bencher) {
        let mut context = Context::new();
        b.iter(|| {
            let mut res: Vec<Bn> = vec![];
            for n in 0..30 + 1 {
                let b = bernoulli(n, &mut context);
                res.push(Bn {
                    value: b.clone(),
                    index: n as i32,
                });
            }
        });
    }

    #[bench]
    fn bench_bernoulli_iter(b: &mut Bencher) {
        b.iter(|| {
            let context = Context::new();
            let _res = context.take(30 + 1).collect::<Vec<_>>();
        });
    }
}
