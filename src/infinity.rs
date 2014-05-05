// Implements http://rosettacode.org/wiki/Infinity

#[cfg(not(test))]
fn main() {
    let inf : f32 = Float::infinity();
    println!("{}", inf);
}
