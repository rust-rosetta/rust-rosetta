// Implements http://rosettacode.org/wiki/Fibonacci_sequence
#[cfg(not(test))]
fn main() {
    let fns=vec![
        (fib_recursive, "recursive implementation"),
        (fib_tail_recursive, "tail recursive implementation"),
        (fib_iterative, "iterative implementation")];

    for (f, desc) in fns.move_iter() {
        let r: Vec<u64> = range(0u64, 10).map(|i| f(i)).collect();
        println!("{}:\n {}\n", desc, r);
    }
}

// Fibonacci "classic" recursive version
// not tail recursive (it's going to blow the stack
// for n too high
fn fib_recursive(n: u64) -> u64 {
    match n {
        n if n<2 => n,
        m => fib_recursive(m - 1) + fib_recursive(m-2)
    }
}

// tail recursive version
fn fib_tail_recursive(n: u64) -> u64 {
        fn in_fib(n : u64, current : u64, next : u64) -> u64 {
            match n {
                0 => current,
                _ => in_fib(n - 1, next, current + next)
            }
        }
        in_fib(n, 0, 1)
}

// iterative version
fn fib_iterative(n: u64) -> u64 {
    let (mut fib1, mut fib2) = (0u64, 1u64);
    let mut fib = n;

    for _ in range(1u64,n) {
        fib = fib1 + fib2;
        fib1 = fib2;
        fib2 = fib;
    }
    fib
}

#[cfg(test)]
fn tester(f: fn(u64)->u64) -> bool {
    let exp = [0u64,1,1,2,3,5,8,13,21,34];
    for i in range(0u, 10) {
        let ret=f(i as u64);
        assert_eq!(ret, exp[i]);
    }
    true
}

#[test]
fn fib_values() {
    let fns=vec![fib_recursive, fib_tail_recursive, fib_iterative];
    fns.move_iter()
        .advance(|f| tester(f));
}