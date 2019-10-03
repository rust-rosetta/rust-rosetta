// IO error is used here just as an example of an already existing
// Error
use std::io::{Error, ErrorKind};

// Rust technically doesn't have exception, but different
// types of error handling. Here are two examples of results.

fn valid_function() -> Result<usize, Error> {
    Ok(100)
}

fn errored_function() -> Result<usize, Error> {
    Err(Error::new(ErrorKind::Other, "Something wrong happened."))
}

// This should happen only when an unrecoverable error happened
fn panicking_function() {
    panic!("Unrecoverable state reached");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_function() {
        let result = match valid_function() {
            Ok(number) => number,
            Err(_) => panic!("This is not going to happen"),
        };
        assert_eq!(result, 100);
    }

    #[test]
    fn test_errored_function() {
        let result = match errored_function() {
            Ok(_) => panic!("This is not going to happen"),
            Err(e) => {
                assert_eq!(e.to_string(), "Something wrong happened.");
                0
            }
        };
        assert_eq!(result, 0);
    }

    #[test]
    #[should_panic]
    fn test_panicking_function() {
        panicking_function();
    }
}
