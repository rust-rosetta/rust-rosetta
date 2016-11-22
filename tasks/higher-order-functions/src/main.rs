fn plain_function() {
    println!("regular function");
}

fn higher_order<F>(cb: F)
    where F: Fn()
{
    cb();
}

fn main() {
    higher_order(plain_function);
    higher_order(|| println!("a closure"));
}
