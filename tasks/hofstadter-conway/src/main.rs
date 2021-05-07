struct HofstadterConway {
    current: usize,
    sequence: Vec<usize>,
}

impl HofstadterConway {
    /// Define a constructor for the struct.
    fn new() -> HofstadterConway {
        HofstadterConway {
            current: 0,
            sequence: vec![1, 1],
        }
    }
}

impl Default for HofstadterConway {
    fn default() -> Self {
        Self::new()
    }
}

/// Implement the hofstadter q iteration sequence.
impl Iterator for HofstadterConway {
    type Item = usize;

    /// This gets called to fetch the next item of the iterator.
    fn next(&mut self) -> Option<usize> {
        let max_index = self.sequence.len() - 1;
        let last_value = self.sequence[max_index];

        if self.current > max_index {
            let new_x = self.sequence[last_value - 1] + self.sequence[max_index - last_value + 1];
            self.sequence.push(new_x);
        }
        self.current += 1;
        Some(self.sequence[self.current - 1])
    }
}

#[allow(clippy::cast_precision_loss)]
fn main() {
    let mut hof = HofstadterConway::new();
    let mut winning_num = 0_usize;

    for p in 0..20 {
        let max_hof = (2_usize.pow(p)..2_usize.pow(p + 1))
            .map(|n| (n, hof.next().unwrap() as f64 / n as f64))
            .fold(f64::NAN, |a, (n, b)| {
                if b >= 0.55 {
                    winning_num = n;
                }
                a.max(b)
            });

        println!("2^{:>2}-2^{:>2}, {:>.8}", p, p + 1, max_hof);
    }

    println!("Winning number: {}", winning_num);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hofsadter_conway() {
        let hofstadter_sequence = HofstadterConway::new().take(10).collect::<Vec<_>>();
        assert_eq!(hofstadter_sequence, vec![1, 1, 2, 2, 3, 4, 4, 4, 5, 6]);
    }
}
