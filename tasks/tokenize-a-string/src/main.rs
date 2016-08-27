// http://rosettacode.org/wiki/Tokenize_a_string

fn main() {
    let s = "Hello,How,Are,You,Today";
    let tokens: Vec<&str> = s.split(',').collect();
    println!("{}", tokens.join("."));
}
