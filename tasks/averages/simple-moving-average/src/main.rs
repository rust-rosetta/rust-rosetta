use std::error::Error;
use std::fmt;

struct MovingAverage {
    period: u32,
    list: Vec<f32>,
}

#[derive(Debug)]
struct MovingAverageError {
    entries: usize,
    period: u32,
}

impl fmt::Display for MovingAverageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "currently only have {} entries, but period is {}",
            self.entries, self.period
        )
    }
}

impl Error for MovingAverageError {}

impl MovingAverage {
    fn new(period: u32) -> MovingAverage {
        if period > 0 {
            MovingAverage {
                period,
                list: Vec::new(),
            }
        } else {
            panic!("Cannot have a period less than or equal to 0");
        }
    }

    fn append(&mut self, value: f32) {
        self.list.push(value);
        if self.list.len() > self.period as usize {
            self.list.remove(0);
        }
    }

    fn calculate(&self) -> Result<f32, MovingAverageError> {
        if self.list.len() < self.period as usize {
            Err(MovingAverageError {
                entries: self.list.len(),
                period: self.period,
            })
        } else {
            Ok(self.list.iter().fold(0.0_f32, |l, r| l + r) / (self.period as f32))
        }
    }
}

fn main() {
    let mut ma = MovingAverage::new(3);
    ma.append(1.0);
    ma.append(2.0);
    ma.append(3.0);

    println!("{:?}", ma.calculate());
}

#[allow(clippy::float_cmp)]
#[test]
fn test_ma() {
    let mut ma = MovingAverage::new(3);
    assert!(ma.calculate().is_err());
    ma.append(1.0);
    assert!(ma.calculate().is_err());
    ma.append(2.0);
    assert!(ma.calculate().is_err());
    ma.append(3.0);
    assert_eq!(ma.calculate().unwrap(), 2.0);
    ma.append(1.0);
    assert_eq!(ma.calculate().unwrap(), 2.0);
    ma.append(8.0);
    assert_eq!(ma.calculate().unwrap(), 4.0);
}
