//! Modeled after [the Scala solution]
//!
//! [the Scala solution]: http://rosettacode.org/wiki/24_game/Solve#Scala

#![feature(slice_patterns)]

extern crate num;
extern crate permutohedron;

use num::rational::{Ratio, Rational};
use num::traits::Zero;
use permutohedron::Heap;

/// convenience macro to create a fixed-sized vector of rationals by writing `rational![1, 2, ...]`
/// instead of `[Ratio::<isize>::from_integer(1), Ratio::<isize>::from_integer(2), ...]`
macro_rules! rationals(
    ($($e:expr),+) => ([$(Ratio::<isize>::from_integer($e)),+])
);

fn main() {
    let mut r = rationals![1, 3, 7, 9];
    let sol = solve(&mut r[..], 24).unwrap_or("no solution found".to_string());
    println!("{}", sol);
}

/// for a vector of rationals r, find the combination of arithmentic operations that yield
/// `target_val` as a result (if such combination exists)
fn solve(r: &mut [Rational], target_val: isize) -> Option<String> {
    // need to sort because next_permutation()
    // returns permutations in lexicographic order
    r.sort();
    loop {
        let all_ops = compute_all_operations(r);
        for &(res, ref ops) in &all_ops {
            if res == Ratio::from_integer(target_val) {
                return Some(ops.to_string());
            }
        }
        let mut perm = Heap::new(r);
        if perm.next_permutation() == None {
            return None;
        }
    }
}

/// applies all the valid combinations of + - * and / to the numbers in l and for each combination
/// creates a tuple with the result and the expression in String form returns all (result,
/// expression in string form) results in a vector
fn compute_all_operations(l: &[Rational]) -> Vec<(Rational, String)> {
    match l {
        &[] => vec![],
        &[x] => vec![(x, (format!("{}", x)))],
        &[x, ref rest..] => {
            let mut rt = Vec::new();
            for &(y, ref exp) in &compute_all_operations(rest) {
                let mut sub = vec![(x * y, "*"), (x + y, "+"), (x - y, "-")];
                if y != Zero::zero() {
                    sub.push((x / y, "/"));
                }
                for &(z, ref op) in &sub {
                    let aux = (z, (format!("({} {} {})", x, op, exp)));
                    rt.push(aux);
                }
            }
            rt
        }
    }
}

#[test]
fn test_rationals_macro() {
    assert_eq!(// without the rationals! macro
               [Ratio::from_integer(1),
                Ratio::from_integer(2),
                Ratio::from_integer(3),
                Ratio::from_integer(4)],
    // with the rationals! macro
               (rationals![1, 2, 3, 4]));
}

#[test]
#[ignore]
fn test_solve() {
    let mut r = rationals![1, 3, 7, 9];
    assert_eq!(solve(&mut r[..], 24),
               Some("(9 / (3 / (1 + 7)))".to_string()));
}
