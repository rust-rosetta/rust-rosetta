extern crate rand;

use std::fmt::{self, Display};

use rand::seq;

const NUMBER_OF_DIGITS: usize = 4;

/// generates a random `NUMBER_OF_DIGITS`
fn generate_digits() -> Vec<u32> {
    let mut rng = rand::thread_rng();
    seq::sample_iter(&mut rng, (1u32..10), NUMBER_OF_DIGITS).unwrap()
}

/// types of errors we can have when parsing a malformed guess
enum ParseError {
    NotValidDigit,
    ExpectedNumberOfDigits(usize),
    NoDuplicates,
}

/// printable description for each `ParseError`
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::NotValidDigit => Display::fmt("only digits from 1 to 9, please", f),
            ParseError::ExpectedNumberOfDigits(exp) => {
                write!(f, "you need to guess with {} digits", exp)
            }
            ParseError::NoDuplicates => Display::fmt("no duplicates, please", f),
        }
    }
}

/// a well-formed guess string should be like "1543", with `NUMBER_OF_DIGITS` digits, no
/// repetitions, no separators or other characters. Parse the guess string as a `Vec<usize>` or
/// return a `ParseError`. This could trivially return a `[usize; NUMBER_OF_DIGITS]` instead of a
/// `Vec<usize>` and avoid dynamic allocations. However, in the more general case,
/// `NUMBER_OF_DIGITS` would not be a constant, but a runtime configuration (which would make using
/// a stack-allocated array more difficult)
fn parse_guess_string(guess: &str) -> Result<Vec<u32>, ParseError> {
    let mut ret = Vec::with_capacity(NUMBER_OF_DIGITS);

    for (i, c) in guess.char_indices() {
        // check that our guess contains the right number of digits
        if i >= NUMBER_OF_DIGITS {
            return Err(ParseError::ExpectedNumberOfDigits(NUMBER_OF_DIGITS));
        }
        match c.to_digit(10) {
            Some(d) if d > 0 => {
                // the guess should not contain duplicate digits
                if ret.contains(&d) {
                    return Err(ParseError::NoDuplicates);
                }
                ret.push(d);
            }
            _ => return Err(ParseError::NotValidDigit),
        }
    }

    Ok(ret)
}

/// returns a tuple with the count of Bulls and Cows in the guess
fn calculate_score(given_digits: &[u32], guessed_digits: &[u32]) -> (usize, usize) {
    let mut bulls = 0;
    let mut cows = 0;
    for (i, given_digit) in given_digits.iter().enumerate().take(NUMBER_OF_DIGITS) {
        let pos = guessed_digits.iter()
            .position(|&a| a == *given_digit);

        match pos {
            None => (),
            Some(p) if p == i => bulls += 1,
            Some(_) => cows += 1,
        }
    }
    (bulls, cows)
}

fn main() {
    let reader = std::io::stdin();
    loop {
        let given_digits = generate_digits();
        println!("I have chosen my {} digits. Please guess what they are",
                 NUMBER_OF_DIGITS);
        loop {
            let mut guess_string = String::new();
            let _ = reader.read_line(&mut guess_string).unwrap();
            let digits_maybe = parse_guess_string(guess_string.trim());
            match digits_maybe {
                Err(msg) => {
                    println!("{}", msg);
                }
                Ok(guess_digits) => {
                    match calculate_score(&given_digits, &guess_digits) {
                        (NUMBER_OF_DIGITS, _) => {
                            println!("you win!");
                            break;
                        }
                        (bulls, cows) => println!("bulls: {}, cows: {}", bulls, cows),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ParseError;

    /// test we generate `NUMBER_OF_DIGITS` unique digits between 1 and 9
    #[test]
    fn generate_digits() {
        let mut digits = super::generate_digits();
        assert!(digits.iter().all(|&d| d > 0u32));
        digits.sort();
        digits.dedup();
        assert_eq!(digits.len(), super::NUMBER_OF_DIGITS)
    }

    #[test]
    fn parse_guess_string() {
        match super::parse_guess_string("1234") {
            Ok(p) => assert_eq!(p, vec![1, 2, 3, 4]),
            _ => panic!("Failed parsing a valid string"),
        }

        match super::parse_guess_string("0123") {
            Ok(_) => panic!("parsed a string containing a 0"),
            Err(err) => {
                if let ParseError::NotValidDigit = err {
                    ()
                } else {
                    panic!("Expected a NotValidDigit error")
                }
            }
        }

        match super::parse_guess_string("1213") {
            Ok(_) => panic!("parsed a string containing a repeated digit"),
            Err(err) => {
                if let ParseError::NoDuplicates = err {
                    ()
                } else {
                    panic!("Expected a NoDuplicates error")
                }
            }
        }

        match super::parse_guess_string("12354") {
            Ok(_) => panic!("parsed a string longer than 4 digits"),
            Err(err) => {
                if let ParseError::ExpectedNumberOfDigits(4) = err {
                    ()
                } else {
                    panic!("Expected a ExpectedNumberOfDigits error")
                }
            }
        }
    }

    #[test]
    fn calculate_score() {
        assert_eq!(super::calculate_score(&[1, 2, 3, 4], &[1, 2, 3, 4]), (4, 0));
        assert_eq!(super::calculate_score(&[1, 2, 3, 4], &[1, 2, 4, 3]), (2, 2));
        assert_eq!(super::calculate_score(&[1, 2, 3, 4], &[5, 6, 7, 8]), (0, 0));
    }
}
