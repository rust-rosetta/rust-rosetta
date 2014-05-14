//http://rosettacode.org/wiki/Function_definition

// Function taking 2 ints, multply them and return the value

fn multiply(x: int, y: int) -> int
{
  //in Rust a statement is a expression. An expression without semicolon is a return expression
  x * y //equivalent "return x * y;"
}

//generic version of multiply
fn multiply_gen<T: Mul<T, T>>(x: T, y: T) -> T
{
  x * y
}

#[test]
fn test_multiply_gen()
{
  assert_eq!(multiply_gen(2,2), 4);
}

#[test]
fn test_multiply()
{
  assert_eq!(multiply(2,2), 4);
}

fn main()
{
  println!("2 multiply 4 = {}", multiply(2,4));
  println!("2.0 multiply 4.0 = {}", multiply_gen(2.0, 4.0));
  println!("5.0 multiply 7.0 is {}", multiply_gen(5.0 as f32, 7.0 as f32));
}
