// Repeat the function f, n times.
fn repeat<F>(f: &F, n: u32)
    where F: Fn() {
    for _ in 0..n {
        f();
    }
}

// Repeat the mutable function f, n times.
fn repeat_mut<F>(f: &mut F, n: u32)
    where F: FnMut() {
    for _ in 0..n {
        f();
    }
}

fn static_fn() {
    print!("Static ");
}

fn main() {

    // Repeat a static function.
    repeat(&static_fn, 4);

    println!("");

    // Repeat an anonymous closure.
    repeat(&|| print!("Closure "), 5);

    println!("");

    // Repeat a mutable closure (can modify local variables).
    let mut x = 1;
    println!("X is {}", x);
    repeat_mut(&mut || x = x + 1, 5);
    println!("X is now {}", x);
}

#[test]
fn test_closure() {
    let mut x = 1;

    repeat_mut(&mut || x = x + 1, 5);

    assert_eq!(x, 6);
}