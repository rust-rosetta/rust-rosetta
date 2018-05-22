//! Accepts two command-line arguments

extern crate rand;

use std::collections::HashSet;
use std::env;
use std::process;

use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

fn help() {
    println!("usage: average_loop_length <max_N> <trials>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut max_n: u32 = 20;
    let mut trials: u32 = 1000;

    match args.len() {
        1 => {}
        3 => {
            max_n = args[1].parse().unwrap();
            trials = args[2].parse().unwrap();
        }
        _ => {
            help();
            process::exit(0);
        }
    }

    let mut rng = thread_rng();

    println!(" N    average    analytical    (error)");
    println!("===  =========  ============  =========");
    for n in 1..(max_n + 1) {
        let the_analytical = analytical(n);
        let the_empirical = empirical(n, trials, &mut rng);
        println!(
            " {:>2}     {:3.4}        {:3.4}  ( {:>+1.2}%)",
            n,
            the_empirical,
            the_analytical,
            100.0 * (the_empirical / the_analytical - 1.0)
        );
    }
}

fn factorial(n: u32) -> f64 {
    (1..n + 1).map(f64::from).product()
}

fn analytical(n: u32) -> f64 {
    (1..(n + 1))
        .map(|i| factorial(n) / f64::from(n).powi(i as i32) / factorial(n - i))
        .sum()
}

fn empirical(n: u32, trials: u32, rng: &mut impl Rng) -> f64 {
    let sum: f64 = (0..trials)
        .map(|_trial| {
            let mut seen = HashSet::new();
            let range = Uniform::new_inclusive(1, n);

            seen.insert(1);
            for step in 1..n {
                let item = rng.sample(range);
                let inserted = seen.insert(item);
                if !inserted {
                    return f64::from(step);
                }
            }
            f64::from(n)
        })
        .sum();
    sum / f64::from(trials)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64;

    #[test]
    fn test_factorial() {
        assert!((factorial(10) - 3628800f64).abs() < f64::EPSILON);
    }

    #[test]
    fn test_analytical() {
        assert!((analytical(10) - 3.6602).abs() < 0.0001);
        assert!((analytical(20) - 5.2936).abs() < 0.0001);
    }

    #[test]
    fn test_empirical() {
        let mut rng = thread_rng();
        let emp = empirical(20, 10000, &mut rng);
        let ana = analytical(20);
        assert!((emp / ana - 1.0).abs() < 0.05);
    }
}
