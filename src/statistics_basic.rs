// http://rosettacode.org/wiki/Statistics/Basic
#![feature(iter_arith)]
extern crate rand;

use rand::distributions::{IndependentSample, Range};

fn mean(data: &[f32]) -> Option<f32> {
    if data.is_empty() {
        None
    } else {
        let sum: f32 = data.iter().sum();
        Some(sum / data.len() as f32)
    }
}

fn variance(data: &[f32]) -> Option<f32> {
    if data.is_empty() {
        None
    } else {
        let mean = mean(data).unwrap();
        let mut sum = 0f32;
        for &x in data {
            sum += (x - mean).powi(2);
        }
        Some(sum / data.len() as f32)
    }
}

fn standard_deviation(data: &[f32]) -> Option<f32> {
    if data.is_empty() {
        None
    } else {
        let variance = variance(data).unwrap();
        Some(variance.sqrt())
    }
}

fn print_histogram(width: u32, data: &[f32]) {
    let mut histogram = [0; 10];
    let len = histogram.len() as f32;
    for &x in data {
        histogram[(x * len) as usize] += 1;
    }
    let max_frequency = *histogram.iter().max().unwrap() as f32;
    for (i, &frequency) in histogram.iter().enumerate() {
        let bar_width = frequency as f32 * width as f32 / max_frequency;
        print!("{:3.1}: ", i as f32 / len);
        for _ in 0..bar_width as usize {
            print!("*");
        }
        println!("");
    }
}

fn main() {
    let range = Range::new(0f32, 1f32);
    let mut rng = rand::thread_rng();

    for &number_of_samples in [1000, 10_000, 1_000_000].iter() {
        let mut data = vec![];
        for _ in 0..number_of_samples {
            let x = range.ind_sample(&mut rng);
            data.push(x);
        }
        println!("  Statistics for sample size {}", number_of_samples);
        println!("Mean:               {:?}", mean(&data));
        println!("Variance:           {:?}", variance(&data));
        println!("Standard deviation: {:?}", standard_deviation(&data));
        print_histogram(40, &data);
    }
}
