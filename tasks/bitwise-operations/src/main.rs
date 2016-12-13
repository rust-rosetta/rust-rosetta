fn main() {
    let a: u8 = 105;
    let b: u8 = 91;
    println!("a      = {:0>8}", a);
    println!("b      = {:0>8}", b);
    println!("a | b  = {:0>8}", a | b);
    println!("a & b  = {:0>8}", a & b);
    println!("a ^ b  = {:0>8}", a ^ b);
    println!("!a     = {:0>8}", !a);
    println!("a << 3 = {:0>8}", a >> 3);
    println!("a >> 3 = {:0>8}", a << 3);
}
