use std::fmt::{Display, Formatter, Result};
use std::process::exit;

struct Coefficient(usize, f64);

impl Display for Coefficient {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let i = self.0;
        let c = self.1;

        if c == 0. {
            return Ok(());
        }

        write!(
            f,
            " {} {}e({})",
            if c < 0. {
                "-"
            } else if f.alternate() {
                " "
            } else {
                "+"
            },
            if (c.abs() - 1.).abs() < f64::EPSILON {
                "".to_string()
            } else {
                c.abs().to_string() + "*"
            },
            i + 1
        )
    }
}

fn usage() {
    println!("Usage: display-linear-combination a1 [a2 a3 ...]");
}

fn linear_combination(coefficients: &[f64]) -> String {
    let mut string = String::new();

    let mut iter = coefficients.iter().enumerate();

    // find first nonzero argument
    loop {
        match iter.next() {
            Some((_, &c)) if c == 0. => {
                continue;
            }
            Some((i, &c)) => {
                string.push_str(format!("{:#}", Coefficient(i, c)).as_str());
                break;
            }
            None => {
                string.push('0');
                return string;
            }
        }
    }

    // print subsequent arguments
    for (i, &c) in iter {
        string.push_str(format!("{}", Coefficient(i, c)).as_str());
    }

    string
}

fn main() {
    let mut coefficients = Vec::new();
    let mut args = std::env::args();

    args.next(); // drop first argument

    // parse arguments into floats
    for arg in args {
        let c = arg.parse::<f64>().unwrap_or_else(|e| {
            eprintln!("Failed to parse argument \"{}\": {}", arg, e);
            exit(-1);
        });
        coefficients.push(c);
    }

    // no arguments, print usage and exit
    if coefficients.is_empty() {
        usage();
        return;
    }

    println!("{}", linear_combination(&coefficients));
}

#[cfg(test)]
mod tests {
    use super::linear_combination;

    #[test]
    fn test01() {
        assert_eq!(
            "   e(1) + 2*e(2) + 3*e(3)",
            linear_combination(&vec![1., 2., 3.])
        )
    }

    #[test]
    fn test02() {
        assert_eq!(
            "   e(2) + 2*e(3) + 3*e(4)",
            linear_combination(&vec![0., 1., 2., 3.])
        )
    }

    #[test]
    fn test03() {
        assert_eq!(
            "   e(1) + 3*e(3) + 4*e(4)",
            linear_combination(&vec![1., 0., 3., 4.])
        )
    }

    #[test]
    fn test04() {
        assert_eq!("   e(1) + 2*e(2)", linear_combination(&vec![1., 2., 0.]))
    }

    #[test]
    fn test05() {
        assert_eq!("0", linear_combination(&vec![0., 0., 0.]))
    }

    #[test]
    fn test06() {
        assert_eq!("0", linear_combination(&vec![0.]))
    }

    #[test]
    fn test07() {
        assert_eq!(
            "   e(1) + e(2) + e(3)",
            linear_combination(&vec![1., 1., 1.])
        )
    }

    #[test]
    fn test08() {
        assert_eq!(
            " - e(1) - e(2) - e(3)",
            linear_combination(&vec![-1., -1., -1.])
        )
    }

    #[test]
    fn test09() {
        assert_eq!(
            " - e(1) - 2*e(2) - 3*e(4)",
            linear_combination(&vec![-1., -2., 0., -3.])
        )
    }

    #[test]
    fn test10() {
        assert_eq!(" - e(1)", linear_combination(&vec![-1.]))
    }
}
