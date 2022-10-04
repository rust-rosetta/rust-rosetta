use rand::distributions::{Distribution, WeightedIndex};
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

/// Generate a weighted distribution for choosing an index.
fn distribution() -> impl Distribution<usize> {
    WeightedIndex::new(DATA.iter().map(|(_, p)| *p)).unwrap()
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
    let counts = take_samples(&mut thread_rng(), &distribution());
    print_mapping(&counts);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn deviation() {
        let mut rng = thread_rng();
        let dist = distribution();

        let counts = take_samples(&mut rng, &dist);
        for (&(_, expected), &count) in DATA.iter().zip(counts.iter()) {
            let real = count as f64 / SAMPLES as f64;
            let dev = (1.0 - real / expected).abs();
            assert!(dev < 0.01, "{}", dev);
        }
    }
}
