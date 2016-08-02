extern crate num;

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Mul, Neg, Sub, Div};

use num::{Zero, One};

fn main() {
    for p in perfect_numbers(1 << 19) {
        println!("{} is perfect", p);
    }
}

fn perfect_numbers(max: i64) -> Vec<i64> {
    let mut ret = Vec::new();
    for candidate in 2..max {
        let mut sum = Frac::secure_new(1, candidate).unwrap();
        let max2 = ((candidate as f64).sqrt().floor()) as i64;

        for factor in 2..max2 + 1 {
            if candidate % factor == 0 {
                sum = sum + Frac::new(1, factor) + Frac::new(factor, candidate);
            }
        }
        if sum == Frac::new(1, 1) {
            ret.push(candidate);
        }
    }
    ret
}

#[derive(Copy, Clone)]
struct Frac {
    num: i64,
    den: i64,
}

fn gcd(m: i64, n: i64) -> i64 {
    let mut t: i64;
    let (mut m, mut n) = (m.abs(), n.abs());
    while n > 0 {
        t = n;
        n = m % n;
        m = t;
    }
    m
}

fn lcm(m: i64, n: i64) -> i64 {
    m.abs() / gcd(m, n) * n.abs()
}

impl Frac {
    /// fails on den=0
    fn new(num: i64, den: i64) -> Frac {
        let (n, d) = match (num, den) {
            (0, _) => (0, 0),
            (n, d) if d < 0 => (-n, -d),
            a => a,
        };

        Frac { num: n, den: d }
    }

    /// does not fail (returns Err on den=0)
    fn secure_new(num: i64, den: i64) -> Result<Frac, String> {
        if den == 0 {
            Err("Error: Division by zero".to_string())
        } else {
            Ok(Frac::new(num, den))
        }
    }

    /// fails on den=0, returns frac already in its reduced form
    fn new_reduced(num: i64, den: i64) -> Frac {
        Frac::new(num, den).reduce()
    }

    /// reduces the fraction to lowest terms
    fn reduce(mut self) -> Frac {
        match self {
            z @ Frac { num: 0, den: 0 } => z,
            _ => {
                let gcd = gcd(self.num, self.den);
                self.num /= gcd;
                self.den /= gcd;
                self
            }
        }
    }
}

impl fmt::Debug for Frac {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.num, self.den) {
            (_, 1) | (0, 0) => write!(f, "{}", self.num),
            (_, _) => write!(f, "{}/{}", self.num, self.den),
        }
    }
}

impl PartialEq for Frac {
    fn eq(&self, other: &Frac) -> bool {
        let (red_a, red_b) = (self.reduce(), other.reduce());
        red_a.num == red_b.num && red_a.den == red_b.den
    }
}

impl Eq for Frac {}

impl PartialOrd for Frac {
    fn partial_cmp(&self, other: &Frac) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Frac {
    fn cmp(&self, other: &Frac) -> Ordering {
        (self.num * other.den).cmp(&(self.den * other.num))
    }
}

impl Neg for Frac {
    type Output = Frac;

    fn neg(self) -> Frac {
        Frac {
            num: -self.num,
            den: self.den,
        }
    }
}

impl Add for Frac {
    type Output = Frac;
    fn add(self, other: Frac) -> Frac {
        let (a, b) = (self.reduce(), other.reduce());
        let m = lcm(a.den, b.den);

        let na = a.num * m / a.den;
        let nb = b.num * m / b.den;
        Frac::new_reduced(na + nb, m)
    }
}

impl Sub for Frac {
    type Output = Frac;
    fn sub(self, other: Frac) -> Frac {
        self + (-other)
    }
}

impl Mul for Frac {
    type Output = Frac;
    fn mul(self, other: Frac) -> Frac {
        Frac::new_reduced(self.num * other.num, self.den * other.den)
    }
}

impl Div for Frac {
    type Output = Frac;
    fn div(self, other: Frac) -> Frac {
        Frac::new_reduced(self.num * other.den, self.den * other.num)
    }
}

impl Zero for Frac {
    fn zero() -> Frac {
        Frac::new(0, 1)
    }

    fn is_zero(&self) -> bool {
        self.num == 0 && self.den != 0
    }
}

impl One for Frac {
    fn one() -> Frac {
        Frac::new(1, 1)
    }
}

#[test]
fn operators() {
    let (a, b) = (Frac::new(1, 2), Frac::new(12, 15));
    assert_eq!(a + b, Frac::secure_new(13, 10).unwrap());
    assert_eq!(b - a, Frac::new(3, 10));
    assert_eq!(a - b, Frac::new(-3, 10));
    assert_eq!(a * b, Frac::new(2, 5));
    assert_eq!(a / b, Frac::new(5, 8));

    let (a, b) = (Frac::new(1, 2), Frac::new(1, 2));
    assert_eq!(a + b, One::one());
    assert_eq!(b - a, Zero::zero());
    assert_eq!(a - b, Zero::zero());
    assert_eq!(a * b, Frac::new(1, 4));
    assert_eq!(a / b, Frac::new(1, 1));
}

#[test]
fn first_perfect_numbers() {
    assert_eq!(perfect_numbers(8150), vec![6, 28, 496, 8128]);
}
