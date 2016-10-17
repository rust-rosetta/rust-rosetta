// Repeat the function f, n times.
fn repeat<F>(f: &F, n: u32)
    where F: Fn() {
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
}
