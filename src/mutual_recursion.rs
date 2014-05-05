// Implements http://rosettacode.org/wiki/Mutual_recursion

#[cfg(not(test))]
fn f(n: int) -> int {
    match n {
        0 => 1,
        _ => n - m(f(n - 1))
    }
}

#[cfg(not(test))]
fn m(n: int) -> int {
    match n {
        0 => 0,
        _ => n - f(m(n - 1))
    }
}

#[cfg(not(test))]
fn main() {
    for i in range(0, 20).map(f) {
        print!("{} ", i);
    }
    println!("")

    for i in range(0, 20).map(m) {
        print!("{} ", i);
    }
    println!("")
}
