// http://rosettacode.org/wiki/24_game/Solve

// modeled after the scala solution
// http://rosettacode.org/wiki/24_game/Solve#Scala
extern crate num;
use num::rational::{Ratio, Rational};
use num::traits::Zero;
// convenience macro to create a fixed-sized vector
// of rationals by writing:
// rational![1, 2, ...] instead of
// [Ratio::<i32>::from_integer(1), Ratio::<i32>::from_integer(2), ...]
macro_rules! rationals(
    ($($e:expr),+) => ([$(Ratio::<i32>::from_integer($e)),+])
);

#[cfg(not(test))]
fn main() {
    let mut r = rationals![1i32, 3, 7, 9];
    let sol = solve(r.as_mut_slice(), 24).unwrap_or("no solution found".to_string());
    println!("{}", sol);
}
// for a vector of rationals r, find the combination of arithmentic
// operations that yield target_val as a result (if such combination exists)
fn solve(r: &mut[Rational], target_val: i32) -> Option<String> {
    //need to sort because next_permutation()
    // returns permutations in lexicographic order
    r.sort();
    loop {
        let all_ops = compute_all_operations(r);
        for &(res, ref ops) in all_ops.iter() {
            if res==Ratio::from_integer(target_val) {return Some(ops.to_string());}
        }
        if ! r.next_permutation() {return None;}
    }
}
// applies all the valid combinations of + - * and / to the
// numbers in l and for each combination creates a tuple
// with the result and the expression in String form
// returns all (result, expression in string form)
// results in a vector
fn compute_all_operations(l: &[Rational]) -> Vec<(Rational, String)> {
    match l {
        []         => vec![],
        [x]  => vec![(x, (format!("{}", x)))],
        [x,rest..] => {
            let mut rt=Vec::new();
            for &(y, ref exp) in compute_all_operations(rest).iter() {
                let mut sub=vec![(x * y, "*"),(x + y, "+"), (x - y, "-")];
                if y != Zero::zero() {sub.push( (x/y, "/")); }
                for &(z, ref op) in sub.iter() {
                    let aux = (z, (format!("({} {} {})", x, op, exp )));
                    rt.push(aux);
                }
            }
            rt
        }
    }
}
#[test]
fn test_rationals_macro() {
    assert_eq!(
    // without the rationals! macro
    [Ratio::from_integer(1),
    Ratio::from_integer(2),
    Ratio::from_integer(3),
    Ratio::from_integer(4)],
    // with the rationals! macro
    (rationals![1i, 2, 3, 4]));
}

#[test]
fn test_solve() {
    let mut r = rationals![1i, 3, 7, 9];
    assert_eq!(
        solve(r.as_mut_slice(), 24),
        Some("(9 / (3 / (1 + 7)))".to_string()));
}
