use std::mem::swap;

fn main() {
    println!("Same type:");
    let mut thing_one = "The First String";
    let mut thing_two = "The Second String";
    println!("Thing 1: {}, Thing 2: {}", thing_one, thing_two);
    swap(&mut thing_one, &mut thing_two);
    println!("Thing 1: {}, Thing 2: {}", thing_one, thing_two);
}
