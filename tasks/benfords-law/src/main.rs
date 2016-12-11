//! Contributed by Gavin Baker <gavinb@antonym.org>

use std::fs::File;
use std::io::{BufReader, BufRead};

/// Calculate the expected frequency of a digit according to Benford's Law
fn benford_freq(d: u64) -> f32 {
    assert!(d >= 1 && d <= 9);

    (1.0 + 1.0 / (d as f32)).log10()
}

/// Returns the leading digit of any number
fn first_digit_of(n: u64) -> usize {
    let mut d = n;
    while d > 9 {
        d /= 10;
    }
    d as usize
}

/// Count frequency table using the first digit of each number in a vector
fn benford_distrib(numbers: &[u64]) -> Vec<f32> {

    // Counts

    let mut counts = [0u64; 10];

    for num in numbers {
        let first = first_digit_of(*num);
        counts[first] += 1;
    }

    // Frequencies

    let mut freq = vec![0f32; 10];

    for digit in 1..10 {
        freq[digit] = counts[digit] as f32 / numbers.len() as f32;
    }

    freq
}

fn main() {
    // Calculate expected frequencies of all digits according to Benford's Law
    let mut expected_distrib = [0f32; 10];
    for digit in &mut expected_distrib {
        *digit = benford_freq(*digit as u64);
    }

    // Load data from the Fibonacci sequence
    let file = BufReader::new(File::open("resources/fib1000.txt").unwrap());

    let fibs: Vec<u64> = file.lines()
        .map(|x| {
            let s = x.unwrap();
            s[0..1].parse::<u64>().unwrap()
        })
        .collect();

    // Calculate freuencies of first digits in test data
    let found_distrib = benford_distrib(&fibs[..]);

    // Print the stats to compare actual vs expected
    println!("\nBenford's Law - Digit Distribution");
    println!("\nFirst 1000 Numbers in the Fibonacci Sequence\n");
    println!("digit    expect     found     delta");
    for digit in 1..10 {
        let expected_pc = expected_distrib[digit] * 100.0;
        let found_pc = found_distrib[digit] * 100.0;
        let delta_pc = expected_pc - found_pc;

        println!("{}        {:>4.1}%      {:>4.1}%    {:>5.2}%",
                 digit,
                 expected_pc,
                 found_pc,
                 delta_pc);
    }
}
