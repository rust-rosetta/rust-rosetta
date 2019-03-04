#![feature(proc_macro_hygiene)]
#![feature(use_extern_macros)]

extern crate factorial_macro;

use factorial_macro::factorial;

fn main() {
    // we can invoke factorial_10! as a regular macro
    println!("{}", factorial!(10));
}

#[test]
fn output() {
    // just testing the output
    // I can't prove programmatically that factorial_10 is actually
    // calculated at compile time
    assert_eq!(factorial!(10), 3628800);
}
