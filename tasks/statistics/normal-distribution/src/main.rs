//! Rust rosetta example for normal distribution

use rand_distr::{Distribution, Normal};

/// Returns the mean of the provided samples
///
/// ## Arguments
/// * data -- reference to float32 array
fn mean(data: &[f32]) -> Option<f32> {
    let sum: f32 = data.iter().sum();
    Some(sum / data.len() as f32)
}

/// Returns standard deviation of the provided samples
///
/// ## Arguments
/// * data -- reference to float32 array
fn standard_deviation(data: &[f32]) -> Option<f32> {
    let mean = mean(data).expect("invalid mean");
    let sum = data.iter().fold(0.0, |acc, &x| acc + (x - mean).powi(2));
    Some((sum / data.len() as f32).sqrt())
}

/// Prints a histogram in the shell
///
/// ## Arguments
/// * data -- reference to float32 array
/// * maxwidth -- the maxwidth of the histogram in # of characters
/// * bincount -- number of bins in the histogram
/// * ch -- character used to plot the graph
fn print_histogram(data: &[f32], maxwidth: usize, bincount: usize, ch: char) {
    let min_val = data.iter().cloned().fold(f32::NAN, f32::min);
    let max_val = data.iter().cloned().fold(f32::NAN, f32::max);

    let delta = (max_val - min_val) / bincount as f32;

    let mut bins = vec![0; bincount];

    for sample in data {
        let bin = ((sample - min_val) / delta) as usize;

        if bin < bincount {
            bins[bin] += 1;
        }
    }

    let max_bin_value = *bins.iter().max().unwrap();

    println!();
    for (i, freq) in bins.into_iter().enumerate() {
        let i = i as f32;

        let bin_min = min_val + i * delta;
        let bin_max = min_val + (i + 1.0) * delta;

        let bar_width = (((freq as f64) / (max_bin_value as f64)) * (maxwidth as f64)) as u32;
        let bar_as_string = (1..bar_width).fold(String::new(), |b, _| b + &ch.to_string());
        println!(
            "({:>6},{:>6}) |{} {:.2}%",
            format!("{:.2}", bin_min),
            format!("{:.2}", bin_max),
            bar_as_string,
            (freq as f64) * 100.0 / (data.len() as f64)
        );
    }
    println!();
}

/// Runs the demo to generate normal distribution of three different sample sizes
fn main() {
    let expected_mean: f32 = 0.0;
    let expected_std_deviation: f32 = 4.0;
    let normal = Normal::new(expected_mean, expected_std_deviation).unwrap();

    let mut rng = rand::thread_rng();
    for &number_of_samples in &[1000, 10_000, 1_000_000] {
        let data: Vec<f32> = normal
            .sample_iter(&mut rng)
            .take(number_of_samples)
            .collect();
        println!("Statistics for sample size {}:", number_of_samples);
        println!("\tMean: {:?}", mean(&data).expect("invalid mean"));
        println!(
            "\tStandard deviation: {:?}",
            standard_deviation(&data).expect("invalid standard deviation")
        );
        print_histogram(&data, 80, 40, '-');
    }
}

#[cfg(test)]
mod tests {
    use super::{mean, print_histogram, standard_deviation};
    use std::f32;

    fn approx(statistics: Option<f32>, value: f32) -> bool {
        (statistics.unwrap() - value).abs() <= f32::EPSILON
    }

    #[test]
    fn test_mean() {
        assert!(approx(mean(&[1.0]), 1.0));
        assert!(approx(mean(&[1.0, 3.0]), 2.0));
        assert!(approx(mean(&[1.0, 2.0, 3.0]), 2.0));
    }

    #[test]
    fn test_standard_deviation() {
        assert!(approx(standard_deviation(&[0.0]), 0.0));
        assert!(approx(standard_deviation(&[1.0, 1.0, 1.0]), 0.0));
        assert!(approx(
            standard_deviation(&[1.0, 2.0, 3.0]),
            (2f32 / 3f32).sqrt()
        ));
    }

    #[test]
    fn test_print_histogram() {
        print_histogram(&[0.0, 1.0, 2.0, 3.0], 10, 5, '-');
    }
}
