// http://rosettacode.org/wiki/Compile-time_calculation

// syntax extension are not yet stable, so we need to opt-in
// explicitly to the phase feature gate
#![allow(unused_features)]
#![feature(core)]
#![feature(plugin)]
// we use this attribute to mark factorial_plugin as
// a syntax extension. The plugin's code is in src/factorial_plugin.rs
#![plugin(factorial_plugin)]

#[cfg(not(test))]
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
