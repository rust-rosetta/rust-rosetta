// Implements http://rosettacode.org/wiki/Currying

// add_n returns a a boxed unboxed closure.

// Once "unboxed, abstract return types" are
// supported it can be done without the
// heap allocation/trait object indirection

#![feature(box_syntax)]
fn add_n(n : i32) -> Box<Fn(i32) -> i32 + 'static>  {
    box move |&: x| n + x
}

fn main() {
    let adder = add_n(40);
    println!("The answer to life is {}.", adder(2));
}