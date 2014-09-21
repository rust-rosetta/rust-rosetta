// http://rosettacode.org/wiki/Function_composition
#[cfg(not(test))]
use std::f32::consts;

#[cfg(not(test))]
fn main() {
    fn f(x: uint) -> String { x.to_string() }
    fn g(x: f32) -> uint { x as uint }

    // just a silly example
    // turn PI to a uint and then
    // the uint to a String
    // via function composition
    let comp = compose(f, g);
    println!("{}", comp(consts::PI));
}

// the future unboxed closures should
// allow to compose closures, not just bare functions (so that environment can be captured)
// Also returning a proc has the limitation that the composed function can only be called once
fn compose<A, B, C>(f: fn(A) -> B, g: fn(C) -> A) -> proc(C): 'static -> B {
    proc(x: C) {f(g(x))}
}

#[test]
fn test_compose() {
    fn inc(x: uint) -> uint { x + 1 }
    fn mul(x: uint) -> uint { x * 3 }

    let comp = compose(inc, mul);
    assert_eq!(comp(3), 10);
}
