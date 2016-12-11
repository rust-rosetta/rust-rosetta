//! Rust does not have a `do...while` loop. Instead, the keyword `loop` is used with a termination
//! condition.

fn main() {
    let mut x = 0;

    loop {
        x += 1;
        println!("{}", x);

        if x % 6 == 0 {
            break;
        }
    }
}
