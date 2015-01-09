// Implements http://rosettacode.org/wiki/Loops/While


fn main() {
    let mut i = 1024u;
    while i > 0 {
        println!("{:?}", i);
        i /= 2;
    }
}
