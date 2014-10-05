use std::num::zero;

fn horner<T:Num>(cs:&[T], x:T) -> T {
    cs.iter().rev().fold(zero::<T>(), |acc, c| (acc*x) + (*c))
}
 
fn main() {
    println!("{}", horner([-19i, 7, -4, 6], 3i));
}