// http://rosettacode.org/wiki/Boolean_values

fn main() {
    // Rust contains a single boolean type: `bool`, represented by the keywords `true` and `false`.
    // Expressions inside `if` and `while` statements must result in type `bool`. There is no
    // automatic conversion to the boolean type.

    let foo = true;
    if foo {
        println!("foo is {}.", foo);
    }

    let bar = false;
    if !bar {
        println!("bar is {}.", bar);
    }

}
