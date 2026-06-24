struct GoldenRatio {
    current: f64,
}

impl GoldenRatio {
    fn new() -> Self {
        GoldenRatio { current: 1.0 }
    }
}

impl Iterator for GoldenRatio {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let phi = self.current;
        self.current = 1.0 + 1.0 / phi;
        Some(phi)
    }
}

pub fn golden_ratio(tolerance: f64) -> (f64, u32, f64) {
    let true_phi: f64 = (1.0 + 5.0_f64.sqrt()) / 2.0;

    let mut seq = GoldenRatio::new();

    let mut prev_phi = seq.next().unwrap();

    for (i, cur_phi) in seq.enumerate() {
        if (cur_phi - prev_phi).abs() <= tolerance {
            let err_margin = (cur_phi - true_phi).abs();
            return (cur_phi, i as u32 + 1, err_margin);
        }
        prev_phi = cur_phi;
    }

    unreachable!()
}

fn main() {
    let tolerance = 1e-5;
    let (phi, iterations, error) = golden_ratio(tolerance);

    println!("Target tolerance: {}", tolerance);
    println!("Final value of φ: {}", phi);
    println!("Total iterations: {}", iterations);
    println!("Absolute error:   {}", error);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator_sequence() {
        // We can test the exact values of the first few manual steps
        let mut seq = GoldenRatio::new();

        // Step 1: 1.0
        assert_eq!(seq.next(), Some(1.0));

        // Step 2: 1 + 1/1.0 = 2.0
        assert_eq!(seq.next(), Some(2.0));

        // Step 3: 1 + 1/2.0 = 1.5
        assert_eq!(seq.next(), Some(1.5));

        // Step 4: 1 + 1/1.5 = 1.666666...
        // For floats, we check if the difference is practically zero rather than using ==
        let fourth = seq.next().unwrap();
        assert!((fourth - 1.6666666666666667).abs() < 1e-10);
    }

    #[test]
    fn test_convergence_logic() {
        let tolerance = 1e-5;
        let (phi, iterations, error) = golden_ratio(tolerance);

        // 1. Ensure the final error is actually within the tolerance we asked for
        assert!(error <= tolerance);

        // 2. We already know from the math that 1e-5 takes exactly 14 iterations
        assert_eq!(iterations, 14);

        // 3. Ensure the calculated phi is strictly greater than 1.6
        // (a basic sanity check that it didn't converge on a random small number)
        assert!(phi > 1.6 && phi < 1.7);
    }

    #[test]
    fn test_dynamic_tolerance() {
        // Test that the engine can handle a much looser tolerance faster
        let loose_tolerance = 1e-1;
        let (_, iterations, error) = golden_ratio(loose_tolerance);

        assert!(error <= loose_tolerance);
        // A looser tolerance should require way fewer than 14 iterations
        assert!(iterations < 14);
    }
}
