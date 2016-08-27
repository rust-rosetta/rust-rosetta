struct StDev {
    len: usize,
    sum: f32,
    sum_sq: f32,
}

impl StDev {
    fn new() -> StDev {
        StDev {
            len: 0,
            sum: 0.0,
            sum_sq: 0.0,
        }
    }

    fn stdev(&mut self, n: f32) -> f32 {
        self.len += 1;
        self.sum += n;
        self.sum_sq += n.powi(2);
        let f32_len = self.len as f32;
        f32::sqrt(self.sum_sq / f32_len - self.sum.powi(2) / f32_len.powi(2))
    }
}

impl Default for StDev {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let test_data: [i32; 8] = [2, 4, 4, 4, 5, 5, 7, 9];
    let mut sd = StDev::new();
    for i in &test_data {
        println!("{}", &sd.stdev(*i as f32));
    }
}

#[test]
fn test_stdev() {
    let eps = 0.000001;
    let mut sd = StDev::new();
    // must be equal with up to eps precision
    assert!((sd.stdev(2.0) - 0.0).abs() < eps);
    assert!((sd.stdev(4.0) - 1.0).abs() < eps);
    assert!((sd.stdev(4.0) - 0.942809).abs() < eps);
    assert!((sd.stdev(4.0) - 0.866025).abs() < eps);
    assert!((sd.stdev(5.0) - 0.979796).abs() < eps);
    assert!((sd.stdev(5.0) - 1.0).abs() < eps);
    assert!((sd.stdev(7.0) - 1.399708).abs() < eps);
    assert!((sd.stdev(9.0) - 2.0).abs() < eps);
}
