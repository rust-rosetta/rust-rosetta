struct R2cf {
    n1: i64,
    n2: i64,
}

// This iterator generates the continued fraction representation from the
// specified rational number.
impl Iterator for R2cf {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        if self.n2 == 0 {
            None
        } else {
            let t1 = self.n1 / self.n2;
            let t2 = self.n2;
            self.n2 = self.n1 - t1 * t2;
            self.n1 = t2;
            Some(t1)
        }
    }
}

fn r2cf(n1: i64, n2: i64) -> R2cf {
    R2cf { n1: n1, n2: n2 }
}

macro_rules! printcf {
    ($x:expr, $y:expr) => (println!("{:?}", r2cf($x, $y).collect::<Vec<_>>()));
}

fn main() {
    printcf!(1, 2);
    printcf!(3, 1);
    printcf!(23, 8);
    printcf!(13, 11);
    printcf!(22, 7);
    printcf!(-152, 77);

    printcf!(14_142, 10_000);
    printcf!(141_421, 100_000);
    printcf!(1_414_214, 1_000_000);
    printcf!(14_142_136, 10_000_000);

    printcf!(31, 10);
    printcf!(314, 100);
    printcf!(3142, 1000);
    printcf!(31_428, 10_000);
    printcf!(314_285, 100_000);
    printcf!(3_142_857, 1_000_000);
    printcf!(31_428_571, 10_000_000);
    printcf!(314_285_714, 100_000_000);
}

#[cfg(test)]
mod tests {
    use std::iter::Iterator;
    use super::r2cf;

    #[test]
    fn test_misc() {
        assert!(Iterator::eq(r2cf(-151, 77), vec![-1, -1, -24, -1, -2]));
        assert!(Iterator::eq(r2cf(22, 7), vec![3, 7]));
        assert!(Iterator::eq(r2cf(23, 8), vec![2, 1, 7]));
    }

    #[test]
    fn test_sqrt2() {
        assert!(Iterator::eq(r2cf(14_142, 10_000), vec![1, 2, 2, 2, 2, 2, 1, 1, 29]));
        assert!(Iterator::eq(r2cf(14_142_136, 10_000_000),
                             vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 6, 1, 2, 4, 1, 1, 2]));
    }

    #[test]
    fn test_pi() {
        assert!(Iterator::eq(r2cf(31, 10), vec![3, 10]));
        assert!(Iterator::eq(r2cf(314, 100), vec![3, 7, 7]));
        assert!(Iterator::eq(r2cf(3_142, 1_000), vec![3, 7, 23, 1, 2]));
    }
}
