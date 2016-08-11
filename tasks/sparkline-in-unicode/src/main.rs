extern crate regex;

use std::f64;

use regex::Regex;

const BARS: &'static str = "▁▂▃▄▅▆▇█";

fn read_samples(input: &'static str) -> Vec<f64> {
    let regex = Regex::new(r"[\s,]+").unwrap();

    input.split(&regex).map(|value| value.parse().unwrap()).collect()
}

fn sparkline(samples: &[f64]) -> String {
    let min = samples.iter().cloned().fold(f64::NAN, f64::min);
    let max = samples.iter().cloned().fold(f64::NAN, f64::max);

    let diff = (max - min) / ((BARS.chars().count() - 1) as f64);

    samples.iter()
        .map(|sample| (sample - min) / diff)
        .map(|idx| BARS.chars().nth(idx as usize).unwrap())
        .collect()
}

fn main() {
    let samples = "1 2 3 4 5 6 7 8 7 6 5 4 3 2 1";
    println!("{}", &samples);
    println!("{}", sparkline(&read_samples(&samples)));

    let samples = "1.5, 0.5 3.5, 2.5 5.5, 4.5 7.5, 6.5";
    println!("{}", &samples);
    println!("{}", sparkline(&read_samples(&samples)));
}

#[cfg(test)]
mod tests {
    use super::{read_samples, sparkline};

    #[test]
    fn simple() {
        assert_eq!("▁▂▃▄▅▆▇█▇▆▅▄▃▂▁",
                   sparkline(&read_samples("1 2 3 4 5 6 7 8 7 6 5 4 3 2 1")));
    }

    #[test]
    fn complex() {
        assert_eq!("▂▁▄▃▆▅█▇",
                   sparkline(&read_samples("1.5, 0.5 3.5, 2.5 5.5, 4.5 7.5, 6.5")));
    }
}
