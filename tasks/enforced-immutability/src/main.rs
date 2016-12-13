#![allow(unused_variables)]

fn main() {
    // Rust let bindings are immutable by default.
    let x = 3;
    // This will raise a compiler error:
    // x += 2;  //~ ERROR cannot assign to immutable borrowed content `*y`

    // You must declare a variable mutable explicitly:
    let mut x = 3;

    // Similarly, references are immutable by default e.g.
    // The following lines would raise a compiler error. Even though x is mutable, y is an
    // immutable reference.
    // let y = &x;
    // *y += 2; //~ ERROR cannot borrow `x` as mutable because it is also borrowed as immutable

    let y = &mut x;
    *y += 2;    // Works

    // Note that though y is now a mutable reference, y itself is still immutable e.g.
    // let mut z = 5;
    // y = &mut z; //~ ERROR re-assignment of immutable variable `y`
}
