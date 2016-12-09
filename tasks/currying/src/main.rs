/// `add_n` returns a boxed closure.
///
/// TODO: Once "unboxed, abstract return types" are
/// supported it can be done without the
/// heap allocation/trait object indirection
fn add_n(n: i32) -> Box<Fn(i32) -> i32> {
    Box::new(move |x| n + x)
}

fn main() {
    let adder = add_n(40);
    println!("The answer to life is {}.", adder(2));
}
