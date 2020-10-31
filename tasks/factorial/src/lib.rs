/// Calculate the factorial using recursion
pub fn factorial_recursive(n: usize) -> usize {
    match n {
        0 => 1,
        _ => n * factorial_recursive(n - 1),
    }
}

/// Calculate the factorial using a fold
pub fn factorial_iterative(n: usize) -> usize {
    (1..n + 1).product()
}

/// Calculate the factorial using a for loop
pub fn factorial_loop(n: usize) -> usize {
    let mut fac = 1;
    for x in 1..n + 1 {
        fac *= x;
    }
    fac
}

#[cfg(test)]
mod tests {
    use super::{factorial_iterative, factorial_loop, factorial_recursive};

    /// Tests
    fn t(f: fn(usize) -> usize) {
        assert_eq!(f(0), 1);
        assert_eq!(f(1), 1);
        assert_eq!(f(2), 2);
        assert_eq!(f(3), 6);
        assert_eq!(f(4), 24);
        assert_eq!(f(5), 120);
        assert_eq!(f(6), 720);
        assert_eq!(f(7), 5040);
        assert_eq!(f(8), 40320);
        assert_eq!(f(9), 362880);
        assert_eq!(f(10), 3628800);
    }

    #[test]
    fn test_fac_recursive() {
        t(factorial_recursive as fn(usize) -> usize)
    }

    #[test]
    fn test_fac_iterative() {
        t(factorial_iterative as fn(usize) -> usize)
    }

    #[test]
    fn test_fac_loop() {
        t(factorial_loop as fn(usize) -> usize)
    }
}
