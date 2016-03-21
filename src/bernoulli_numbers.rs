// http://rosettacode.org/wiki/Bernoulli_numbers

#![feature(test)]

extern crate num;
extern crate test;

use num::bigint::{BigInt, ToBigInt};
use num::rational::{BigRational};
use std::env;
use std::ops::{Mul, Sub};
use std::process;


struct Context {
    n: usize,
    bigone_const: BigInt,
    bigrat_const: BigRational,
    a: Vec<BigRational>,
    index: i32    // For iterator
}

impl Context {
    pub fn new(up_to: usize) -> Context {
        let bigone = 1.to_bigint().unwrap();
        let bigrat = BigRational::new(bigone.clone(), bigone.clone());
        let a_vec: Vec<BigRational> = vec![bigrat.clone(); up_to + 1];
        Context {
            n: up_to,
            bigone_const: bigone,
            bigrat_const: bigrat,
            a: a_vec,
            index: -1
        }
    }
}

fn help() {
    println!("Usage: bernoulli_numbers <up_to>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut up_to: usize = 60;

    match args.len() {
        1 => {},
        2 => {
            up_to = args[1].parse::<usize>().unwrap();
        },
        _ => {
            help();
            process::exit(0);
        }
    }

    let mut context = Context::new(up_to);

    let mut res: Vec<(usize, BigRational)> = vec![];
    let mut widths: Vec<u32> = vec![];

    //for n in 0..up_to + 1 {
    //    let b = bernoulli(n, &mut context);
    //println!("got {} for {}", b, n);
    let mut res = context.take(up_to + 1).collect::<Vec<_>>();
    //let x = context.take(up_to + 1).count();
    /*for r in context.take(up_to + 1) {
        res.push(r.clone());
        widths.push(r.1.numer().to_string().len());
    }*/

    //let width = widths.iter().max().unwrap();
    let width = 10;
    for r in res.iter().filter(|r| r.0 % 2 == 0) {
        println!("B({:>2}) = {:>2$} / {denom}", r.0, r.1.numer(), width, denom = r.1.denom());
    }
}

// Implementation with no reused calculations.
fn bernoulli_naive(n: usize, c: &mut Context) -> BigRational {

    for m in 0..n + 1 {
        c.a[m] = BigRational::new(c.bigone_const.clone(), (m + 1).to_bigint().unwrap());
        for j in (1..m + 1).rev() {
            c.a[j - 1] = (c.a[j - 1].clone().sub(c.a[j].clone())).mul(
                BigRational::new(j.to_bigint().unwrap(), c.bigone_const.clone())
            );
        }
    }
    c.a[0].reduced()
}

// Implementation with reused calculations (does not require sequential calls).
// This is ~100x faster.
fn bernoulli(n: usize, c: &mut Context) -> BigRational {
    for i in 1..n + 1 {
        if c.a[i].eq(&c.bigrat_const) {
            c.a[i] = BigRational::new(c.bigone_const.clone(), (i + 1).to_bigint().unwrap());
            for j in (1..i + 1).rev() {
                c.a[j - 1] = (c.a[j - 1].clone().sub(c.a[j].clone())).mul(
                    BigRational::new(j.to_bigint().unwrap(), c.bigone_const.clone())
                );
            }
        }
    }
    c.a[0].reduced()
}

// Iterator implementation.
impl Iterator for Context {
    type Item = (usize, BigRational);

    fn next(&mut self) -> Option<(usize, BigRational)> {
        self.index += 1;
        Some((self.index as usize, self.bigrat_const.clone()))
    }
}


#[cfg(test)]
mod tests {
    use super::{Context, bernoulli, bernoulli_naive};
    use num::rational::{BigRational};
    use std::str::FromStr;
    use test::Bencher;

    #[test]
    fn test_bernoulli_naive() {
        let mut context = Context::new(60);
    }

    #[test]
    fn test_bernoulli_naive() {
        let mut context = Context::new(60);
        assert_eq!(bernoulli_naive(60, &mut context), BigRational::new(
                FromStr::from_str("-1215233140483755572040304994079820246041491").unwrap(),
                FromStr::from_str("56786730").unwrap()
            )
        );
    }

    #[test]
    fn test_bernoulli() {
        let mut context = Context::new(60);
        assert_eq!(bernoulli(60, &mut context), BigRational::new(
                FromStr::from_str("-1215233140483755572040304994079820246041491").unwrap(),
                FromStr::from_str("56786730").unwrap()
            )
        );
    }

    #[bench]
    fn bench_bernoulli_naive(b: &mut Bencher) {
        let mut context = Context::new(30);
        b.iter(|| {
            let mut res: Vec<(usize, BigRational)> = vec![];
            for n in 0..30 + 1 {
                let b = bernoulli_naive(n, &mut context);
                res.push((n, b.clone()));
            }
        });
    }

    #[bench]
    fn bench_bernoulli(b: &mut Bencher) {
        let mut context = Context::new(30);
        b.iter(|| {
            let mut res: Vec<(usize, BigRational)> = vec![];
            for n in 0..30 + 1 {
                let b = bernoulli(n, &mut context);
                res.push((n, b.clone()));
            }
        });
    }
}
