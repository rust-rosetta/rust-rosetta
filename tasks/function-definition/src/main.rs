use std::ops::Mul;

/// Function taking 2 ints, multply them and return the value
fn multiply(x: i32, y: i32) -> i32 {
    // In Rust a statement is a expression. An expression at the end of a
    // function without semicolon is a return expression
    x * y //equivalent "return x * y;"
}

/// generic version of multiply
fn multiply_gen<T: Mul<Output = T>>(x: T, y: T) -> T {
    x * y
}

#[test]
fn test_multiply_gen() {
    assert_eq!(multiply_gen(2i32, 2), 4);
}

#[test]
fn test_multiply() {
    assert_eq!(multiply(2i32, 2), 4);
}

fn main() {
    println!("2 multiply 4 = {}", multiply(2i32, 4));
    println!("2.0 multiply 4.0 = {}", multiply_gen(2.0f32, 4.0));
    println!("5.0 multiply 7.0 is {}",
             multiply_gen(5.0 as f32, 7.0 as f32));
}
