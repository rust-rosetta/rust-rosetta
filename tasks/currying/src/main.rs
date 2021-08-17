/// `add_n` returns a closure.
fn add_n(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

fn main() {
    let adder = add_n(40);
    println!("The answer to life is {}.", adder(2));
}
