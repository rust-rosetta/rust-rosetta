use bigdecimal::BigDecimal;
use std::str::FromStr;

struct LucasSequence {
    b: BigDecimal,
    prev2: BigDecimal, // x_n-2
    prev1: BigDecimal, // x_n-1
}

impl Iterator for LucasSequence {
    type Item = BigDecimal;

    fn next(&mut self) -> Option<Self::Item> {
        let new_val = &self.b * &self.prev1 + &self.prev2;

        // update struct state using mem::replace as copy trait not in BigDecimal
        self.prev2 = std::mem::replace(&mut self.prev1, new_val);

        Some(self.prev1.clone())
    }
}

impl LucasSequence {
    fn new(b: u32) -> Self {
        LucasSequence {
            b: BigDecimal::from(b),
            prev2: BigDecimal::from_str("1.0").unwrap(),
            prev1: BigDecimal::from_str("1.0").unwrap(),
        }
    }

    fn calc_metallic_ratio(b: u32) -> (String, usize) {
        let seq: Self = Self::new(b);
        let mut iterations: usize = 0;
        let mut last_ratio: String = String::new();

        let mut prev_val: BigDecimal = BigDecimal::from_str("1.0").unwrap();

        for cur_val in seq {
            iterations += 1;

            // Increase precision and divide by previous value of sequence
            let cur_val_highprec = cur_val.with_prec(300);
            let ratio: BigDecimal = &cur_val_highprec / &prev_val;

            // Format and Check convergence
            let cur_ratio = format!("{:.32}", ratio);
            if cur_ratio == last_ratio {
                return (cur_ratio, iterations);
            }

            last_ratio = cur_ratio;
            prev_val = cur_val;
        }

        unreachable!();
    }
}

fn main() {
    println!("--- The Metallic Ratios (b = 0 to 9) ---");

    for b in 0..=9 {
        println!("\nRatio b = {}", b);

        // Task 1: Print the first 15 elements
        // We create a fresh sequence, grab the first two static elements (1, 1),
        // and then dynamically generate the next 13 using your Iterator!
        print!("Sequence: 1, 1");
        let seq = LucasSequence::new(b);
        for val in seq.take(13) {
            print!(", {}", val);
        }
        println!();

        // Task 2: Calculate 32-decimal convergence
        let (final_ratio, iterations) = LucasSequence::calc_metallic_ratio(b);

        println!("Value: {}", final_ratio);
        println!("Iterations required: {}", iterations);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_sequence_generation_b1() {
        // Test the underlying iterator engine for the Fibonacci sequence (b = 1)
        let mut seq = LucasSequence::new(1);

        // Your iterator mathematically starts yielding from the 3rd element (2)
        assert_eq!(seq.next(), Some(BigDecimal::from_str("2.0").unwrap()));
        assert_eq!(seq.next(), Some(BigDecimal::from_str("3.0").unwrap()));
        assert_eq!(seq.next(), Some(BigDecimal::from_str("5.0").unwrap()));
        assert_eq!(seq.next(), Some(BigDecimal::from_str("8.0").unwrap()));
    }

    #[test]
    fn test_sequence_generation_b2() {
        // Test the underlying iterator engine for the Silver Ratio sequence (b = 2)
        let mut seq = LucasSequence::new(2);

        assert_eq!(seq.next(), Some(BigDecimal::from_str("3.0").unwrap()));
        assert_eq!(seq.next(), Some(BigDecimal::from_str("7.0").unwrap()));
        assert_eq!(seq.next(), Some(BigDecimal::from_str("17.0").unwrap()));
    }

    #[test]
    fn test_golden_ratio_convergence() {
        // Test b = 1
        let (ratio, iterations) = LucasSequence::calc_metallic_ratio(1);
        let expected_ratio = "1.61803398874989484820458683436564";

        assert_eq!(ratio, expected_ratio);
        assert_eq!(iterations, 78);
    }

    #[test]
    fn test_silver_ratio_convergence() {
        // Test b = 2
        let (ratio, iterations) = LucasSequence::calc_metallic_ratio(2);
        let expected_ratio = "2.41421356237309504880168872420970";

        assert_eq!(ratio, expected_ratio);
        assert_eq!(iterations, 44);
    }

    #[test]
    fn test_platinum_ratio_convergence() {
        // Test the edge case (b = 0)
        let (ratio, iterations) = LucasSequence::calc_metallic_ratio(0);
        let expected_ratio = "1.00000000000000000000000000000000";

        assert_eq!(ratio, expected_ratio);
        assert_eq!(iterations, 2);
    }
}
