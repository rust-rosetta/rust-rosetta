#![feature(non_ascii_idents)]

// Rust warns about non-snake-case variable names by default, and will suggest `δ` as the variable
// name instead.
#[allow(non_snake_case)]
fn main() {
    let mut Δ: i32 = 1;
    Δ += 1;
    println!("{}", Δ);
}
