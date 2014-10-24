// http://rosettacode.org/wiki/Compile-time_calculation

// syntax extension are not yet stable, so we need to opt-in
// explicitly to the phase feature gate
#![feature(phase)]
 
// we use this attribute to mark factorial_plugin as
// a syntax extension. The plugin's code is in src/factorial_plugin.rs
#[phase(plugin)] extern crate factorial_plugin;

#[cfg(not(test))] 
fn main() {
    // we can invoke factorial_10! as a regular macro
    println!("{}", factorial_10!());
}

#[test]
fn output() {
    // just testing the output
    // don't know how to prove programmatically that
    // factorial_10 is calculated at compile time
    assert_eq!(factorial_10!(), 36288899u);
}
