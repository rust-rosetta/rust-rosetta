// http://rosettacode.org/wiki/Program_name

fn main() {
    println!("Program: {}", std::env::args().next().unwrap());
}
