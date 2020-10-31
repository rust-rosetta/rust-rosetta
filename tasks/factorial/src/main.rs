use factorial::{factorial_iterative, factorial_loop, factorial_recursive};

fn main() {
    let fs = vec![
        ("Recursive", factorial_recursive as fn(usize) -> usize),
        ("Iterative", factorial_iterative as fn(usize) -> usize),
        ("Looooooop", factorial_loop as fn(usize) -> usize),
    ];
    for (name, f) in fs {
        println!("---------\n{}", name);
        for i in 1..10 {
            println!("{}", f(i))
        }
    }
}
