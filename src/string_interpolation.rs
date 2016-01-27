// http://rosettacode.org/wiki/String_interpolation

fn main() {
    let original = "Mary had a X lamb";
    let little = "little";
    let replaced = original.replace("X", little);
    println!("{}", replaced);
}
