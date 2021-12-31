use std::{
    cmp::min,
    convert::{TryFrom, TryInto},
    fmt,
    ops::{Add, Mul, Neg},
    str::FromStr,
};

fn main() -> Result<(), &'static str> {
    let a = BalancedTernary::from_str("+-0++0+")?;
    let b = BalancedTernary::from(-436);
    let c = BalancedTernary::from_str("+-++-")?;
    println!("a = {} = {}", a, i128::try_from(a.clone())?);
    println!("b = {} = {}", b, i128::try_from(b.clone())?);
    println!("c = {} = {}", c, i128::try_from(c.clone())?);

    let d = a * (b + -c);
    println!("a * (b - c) = {} = {}", d, i128::try_from(d.clone())?);

    let a = BalancedTernary::from_str(
        "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++",
    )?;
    assert!(i128::try_from(a).is_err());

    Ok(())
}

#[derive(Clone, Copy, PartialEq)]
enum Trit {
    Zero,
    Pos,
    Neg,
}

impl TryFrom<char> for Trit {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Self::Zero),
            '+' => Ok(Self::Pos),
            '-' => Ok(Self::Neg),
            _ => Err("Invalid character for balanced ternary"),
        }
    }
}

impl From<Trit> for char {
    fn from(x: Trit) -> Self {
        match x {
            Trit::Zero => '0',
            Trit::Pos => '+',
            Trit::Neg => '-',
        }
    }
}

impl Add for Trit {
    // (Carry, Current)
    type Output = (Self, Self);

    fn add(self, rhs: Self) -> Self::Output {
        use Trit::{Neg, Pos, Zero};
        match (self, rhs) {
            (Zero, x) | (x, Zero) => (Zero, x),
            (Pos, Neg) | (Neg, Pos) => (Zero, Zero),
            (Pos, Pos) => (Pos, Neg),
            (Neg, Neg) => (Neg, Pos),
        }
    }
}

impl Mul for Trit {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        use Trit::{Neg, Pos, Zero};
        match (self, rhs) {
            (Zero, _) | (_, Zero) => Zero,
            (Pos, Pos) | (Neg, Neg) => Pos,
            (Pos, Neg) | (Neg, Pos) => Neg,
        }
    }
}

impl Neg for Trit {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Trit::Zero => Trit::Zero,
            Trit::Pos => Trit::Neg,
            Trit::Neg => Trit::Pos,
        }
    }
}

// The vector is stored in reverse from how it would be viewed, as
// operations tend to work backwards
#[derive(Clone)]
struct BalancedTernary(Vec<Trit>);

impl fmt::Display for BalancedTernary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .rev()
                .map(|&d| char::from(d))
                .collect::<String>()
        )
    }
}

impl Add for BalancedTernary {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        use Trit::Zero;

        // Trim leading zeroes
        fn trim(v: &mut Vec<Trit>) {
            while let Some(last_elem) = v.pop() {
                if last_elem != Zero {
                    v.push(last_elem);
                    break;
                }
            }
        }

        if rhs.0.is_empty() {
            // A balanced ternary shouldn't be empty
            if self.0.is_empty() {
                return BalancedTernary(vec![Zero]);
            }
            return self;
        }

        let length = min(self.0.len(), rhs.0.len());
        let mut sum = Vec::new();
        let mut carry = vec![Zero];

        for i in 0..length {
            let (carry_dig, digit) = self.0[i] + rhs.0[i];
            sum.push(digit);
            carry.push(carry_dig);
        }
        // At least one of these two loops will be ignored
        for i in length..self.0.len() {
            sum.push(self.0[i]);
        }
        for i in length..rhs.0.len() {
            sum.push(rhs.0[i]);
        }

        trim(&mut sum);
        trim(&mut carry);

        BalancedTernary(sum) + BalancedTernary(carry)
    }
}

// This version of `Mul` requires an implementation of the `Add` trait
impl Mul for BalancedTernary {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut results = Vec::with_capacity(rhs.0.len());
        for i in 0..rhs.0.len() {
            let mut digits = vec![Trit::Zero; i];
            for j in 0..self.0.len() {
                digits.push(self.0[j] * rhs.0[i]);
            }
            results.push(BalancedTernary(digits));
        }
        #[allow(clippy::suspicious_arithmetic_impl)]
        results
            .into_iter()
            .fold(BalancedTernary(vec![Trit::Zero]), |acc, x| acc + x)
    }
}

impl Neg for BalancedTernary {
    type Output = Self;

    fn neg(self) -> Self::Output {
        BalancedTernary(self.0.iter().map(|&x| -x).collect())
    }
}

impl FromStr for BalancedTernary {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .rev()
            .map(|c| c.try_into())
            .collect::<Result<_, _>>()
            .map(BalancedTernary)
    }
}

impl From<i128> for BalancedTernary {
    fn from(x: i128) -> Self {
        let mut v = Vec::new();
        let mut curr = x;

        loop {
            let rem = curr % 3;

            match rem {
                0 => v.push(Trit::Zero),
                1 | -2 => v.push(Trit::Pos),
                2 | -1 => v.push(Trit::Neg),
                _ => unreachable!(),
            }

            let offset = (rem as f64 / 3.0).round() as i128;
            curr = curr / 3 + offset;

            if curr == 0 {
                break;
            }
        }

        BalancedTernary(v)
    }
}

impl TryFrom<BalancedTernary> for i128 {
    type Error = &'static str;

    fn try_from(value: BalancedTernary) -> Result<Self, Self::Error> {
        value
            .0
            .iter()
            .enumerate()
            .try_fold(0_i128, |acc, (i, character)| {
                let size_err = "Balanced ternary string is too large to fit into 16 bytes";
                let index: u32 = i.try_into().map_err(|_| size_err)?;

                match character {
                    Trit::Zero => Ok(acc),
                    Trit::Pos => 3_i128
                        .checked_pow(index)
                        .and_then(|x| acc.checked_add(x))
                        .ok_or(size_err),
                    Trit::Neg => 3_i128
                        .checked_pow(index)
                        .and_then(|x| acc.checked_sub(x))
                        .ok_or(size_err),
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(BalancedTernary::from_str("+-0++0+").is_ok(), true);
        assert_eq!(BalancedTernary::from_str("+-O++0+").is_err(), true);
        assert_eq!(
            BalancedTernary::from_str(
                "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++"
            )
            .is_ok(),
            true
        );
    }

    #[test]
    fn test_from_int() {
        BalancedTernary::from(0);
        BalancedTernary::from(-5);
        BalancedTernary::from(21);
    }

    #[test]
    fn test_to_int() {
        assert_eq!(
            i128::try_from(BalancedTernary::from_str("+-0++0+").unwrap()),
            Ok(523)
        );
        assert_eq!(
            i128::try_from(BalancedTernary::from_str("-++-0--").unwrap()),
            Ok(-436)
        );
        assert_eq!(
            i128::try_from(BalancedTernary::from_str("-++-0--").unwrap()),
            Ok(-436)
        );
        assert_eq!(
            i128::try_from(BalancedTernary::from_str(
                "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++"
            ).unwrap())
            .is_err(),
            true
        );
    }

    #[test]
    fn test_to_str() {
        assert_eq!(
            BalancedTernary::from_str("+-0++0+").unwrap().to_string(),
            String::from("+-0++0+")
        );
        assert_eq!(
            BalancedTernary::from_str("0").unwrap().to_string(),
            String::from("0")
        );
    }

    #[test]
    fn test_add() {
        let a = BalancedTernary::from(45);
        let b = BalancedTernary::from(2);
        let c = BalancedTernary::from(-2);
        assert_eq!(i128::try_from(a.clone() + b), Ok(47));
        assert_eq!(i128::try_from(c + a), Ok(43));
    }

    #[test]
    fn test_neg() {
        let a = BalancedTernary::from_str("+-0++0+").unwrap();
        assert_eq!((-a).to_string(), String::from("-+0--0-"));
    }

    #[test]
    fn test_mul() {
        let a = BalancedTernary::from(45);
        let b = BalancedTernary::from(20);
        let c = BalancedTernary::from(-20);
        assert_eq!(i128::try_from(a.clone() * b), Ok(900));
        assert_eq!(i128::try_from(c * a), Ok(-900));
    }
}
