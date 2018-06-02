extern crate rand;

use rand::distributions::{Distribution, Weighted, WeightedChoice};
use rand::prelude::*;

const DATA: [(&str, f64); 8] = [
    ("aleph", 1.0 / 5.0),
    ("beth", 1.0 / 6.0),
    ("gimel", 1.0 / 7.0),
    ("daleth", 1.0 / 8.0),
    ("he", 1.0 / 9.0),
    ("waw", 1.0 / 10.0),
    ("zayin", 1.0 / 11.0),
    ("heth", 1759.0 / 27720.0),
];

const SAMPLES: usize = 1_000_000;

/// Generate a mapping to be used by `WeightedChoice`
fn gen_mapping() -> Vec<Weighted<usize>> {
    DATA.iter()
        .enumerate()
        .map(|(i, &(_, p))| Weighted {
            // `WeightedChoice` requires `u32` weights rather than raw probabilities.  For each
            // probability, we convert it to a `u32` weight, and associate it with an index. We
            // multiply by a constant because small numbers such as 0.2 when casted to `u32`
            // become `0`.  This conversion decreases the accuracy of the mapping, which is why we
            // provide an implementation which uses `f64`s for the best accuracy.
            weight: (p * 1_000_000_000.0) as u32,
            item: i,
        })
        .collect()
}

/// Generate a mapping of the raw probabilities
fn gen_mapping_float() -> Vec<f64> {
    // This does the work of `WeightedChoice::new`, splitting a number into various ranges.  The
    // `item` of `Weighted` is represented here merely by the probability's position in the `Vec`.
    let mut running_total = 0.0;
    DATA.iter()
        .map(|&(_, p)| {
            running_total += p;
            running_total
        })
        .collect()
}

/// An implementation of `WeightedChoice` which uses probabilities rather than weights.  Refer to
/// the `WeightedChoice` source for serious usage.
struct WcFloat {
    mapping: Vec<f64>,
}

impl WcFloat {
    fn new(mapping: &[f64]) -> Self {
        Self {
            mapping: mapping.to_vec(),
        }
    }

    // This is roughly the same logic as `WeightedChoice::ind_sample` (though is likely slower)
    fn search(&self, sample_prob: f64) -> usize {
        let idx = self
            .mapping
            .binary_search_by(|p| p.partial_cmp(&sample_prob).unwrap());
        match idx {
            Ok(i) | Err(i) => i,
        }
    }
}

impl Distribution<usize> for WcFloat {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
        // Because we know the total is exactly 1.0, we can merely use a raw float value.
        // Otherwise caching `Range::new(0.0, running_total)` and sampling with
        // `range.ind_sample(&mut rng)` is recommended.
        let sample_prob = rng.gen::<f64>();
        self.search(sample_prob)
    }
}

fn take_samples<R: Rng>(rng: &mut R, wc: &impl Distribution<usize>) -> [usize; 8] {
    let mut counts = [0; 8];
    for _ in 0..SAMPLES {
        let sample = rng.sample(wc);
        counts[sample] += 1;
    }
    counts
}

fn print_mapping(counts: &[usize]) {
    println!("Item   | Expected | Actual   ");
    println!("-------+----------+----------");
    for (&(name, expected), &count) in DATA.iter().zip(counts.iter()) {
        let real = count as f64 / SAMPLES as f64;
        println!("{:06} | {:.6} | {:.6}", name, expected, real);
    }
}

fn main() {
    let mut rng = SmallRng::from_entropy();

    println!("    ~~~ U32 METHOD ~~~");
    let mut mapping = gen_mapping();
    let wc = WeightedChoice::new(&mut mapping);

    let counts = take_samples(&mut rng, &wc);
    print_mapping(&counts);

    println!();

    println!("   ~~~ FLOAT METHOD ~~~");
    // initialize the float version of `WeightedChoice`
    let mapping = gen_mapping_float();
    let wc = WcFloat::new(&mapping);

    let counts = take_samples(&mut rng, &wc);
    print_mapping(&counts);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_sample_logic(samples: usize) {
        let mut rng = SmallRng::from_entropy();

        let mapping = gen_mapping_float();
        let wc = WcFloat::new(&mapping);

        for _ in 0..samples {
            let prob = rng.gen::<f64>();
            let i = wc.search(prob);

            assert!(prob <= mapping[i], "p:{} m:{}", prob, mapping[i]);
            if i != 0 {
                assert!(mapping[i - 1] <= prob, "p:{} m:{}", prob, mapping[i - 1]);
            }
        }
    }

    #[test]
    fn small_logic() {
        test_sample_logic(1000);
    }

    #[test]
    #[ignore]
    fn large_logic() {
        test_sample_logic(100_000_000);
    }

    fn test_deviation<T: Distribution<usize>>(wc: T) {
        let mut rng = SmallRng::from_entropy();

        let counts = take_samples(&mut rng, &wc);
        for (&(_, expected), &count) in DATA.iter().zip(counts.iter()) {
            let real = count as f64 / SAMPLES as f64;
            let dev = (1.0 - real / expected).abs();
            assert!(dev < 0.01, "{}", dev);
        }
    }

    #[test]
    #[ignore]
    fn wcf_deviation() {
        let mapping = gen_mapping_float();
        let wc = WcFloat::new(&mapping);

        test_deviation(wc);
    }

    #[test]
    #[ignore]
    fn wc_deviation() {
        let mut mapping = gen_mapping();
        let wc = WeightedChoice::new(&mut mapping);

        test_deviation(wc);
    }
}
