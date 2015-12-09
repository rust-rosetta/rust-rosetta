// http://rosettacode.org/wiki/Loops/Downward_for

fn main() {
    for i in (1 .. 10 + 1).rev() {
        println!("{}", i);
    }
}
