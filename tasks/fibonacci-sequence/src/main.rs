fn main() {
    let fns = vec![(fib_recursive as fn(u64) -> u64, "recursive"),
                   (fib_tail_recursive as fn(u64) -> u64, "tail recursive"),
                   (fib_iterative as fn(u64) -> u64, "iterative")];

    for (f, desc) in fns {
        let r = (0u64..10).map(f).collect::<Vec<u64>>();
        println!("{} implementation:\n{:?}\n", desc, r);
    }
}

/// Fibonacci "classic" recursive version
/// not tail recursive (it's going to blow the stack for n too high)
fn fib_recursive(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        n => fib_recursive(n - 1) + fib_recursive(n - 2),
    }
}

/// tail recursive version
fn fib_tail_recursive(n: u64) -> u64 {
    fn in_fib(n: u64, current: u64, next: u64) -> u64 {
        match n {
            0 => current,
            n => in_fib(n - 1, next, current + next),
        }
    }

    in_fib(n, 0, 1)
}

/// iterative version
fn fib_iterative(n: u64) -> u64 {
    let (mut cur, mut next) = (0u64, 1u64);

    for _ in 0u64..n {
        let tmp = cur + next;
        cur = next;
        next = tmp;
    }

    cur
}

#[cfg(test)]
mod tests {
    use super::{fib_recursive, fib_iterative, fib_tail_recursive};

    /// helper function to test that all versions of the fib function
    /// return the expected values.
    fn tester(f: fn(u64) -> u64) {
        let exp = [0u64, 1, 1, 2, 3, 5, 8, 13, 21, 34];
        for (i, expected) in (0u64..10).zip(exp.iter()) {
            assert_eq!(f(i), *expected);
        }
    }

    #[test]
    fn fib_values() {
        let fns = vec![fib_recursive as fn(u64) -> u64,
                       fib_tail_recursive as fn(u64) -> u64,
                       fib_iterative as fn(u64) -> u64];
        for &f in &fns {
            tester(f);
        }
    }
}
