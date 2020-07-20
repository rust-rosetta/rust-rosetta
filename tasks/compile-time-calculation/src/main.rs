use static_assertions::const_assert_eq;

const fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

fn main() {
    // We can invoke factorial as a regular function
    println!("{}", factorial(10));
}

// This assertion runs at compile time.
const_assert_eq!(factorial(10), 3628800);
