// http://rosettacode.org/wiki/Function_composition

// TODO: decide what to do for 1.0
// the manual implementation of Fn traits for a struct
// as done here for Composed is not going to be un-feature-gated
// for 1.0 (so it's going to work only with the Rust nightlies)
#![feature(unboxed_closures)]

#[cfg(not(test))]
fn main() {
    use std::f32::consts;

    fn f(x: uint) -> String { x.to_string() }
    fn g(x: f32) -> uint { x as uint }
    
    let comp = Composed::new(f, g);
    
    println!("{}", comp(consts::PI));
}

struct Composed<A, B, C, F1, F2> 
    where F1: Fn(A) -> B, F2: Fn(C) -> A {
    f: F1,
    g: F2
}

impl <A, B, C, F1, F2> Composed<A, B, C, F1, F2> 
    where F1: Fn(A) -> B, F2: Fn(C) -> A {
    fn new(f: F1, g: F2) -> Composed<A, B, C, F1, F2> { Composed{f: f, g: g} }
}

impl<A, B, C, F1, F2> Fn<(C,), B> for Composed<A, B, C, F1, F2> 
    where F1: Fn(A) -> B, F2: Fn(C) -> A {
    extern "rust-call" fn call(&self, (x,): (C,)) -> B { self.f.call((self.g.call((x,)),)) }
}

#[test]
fn test_compose() {
    fn inc(x: uint) -> uint { x + 1 }
    fn mul(x: uint) -> uint { x * 3 }

    let comp = Composed::new(inc, mul);
    assert_eq!(comp(3), 10);
}
