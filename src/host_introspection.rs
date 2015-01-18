// Implements http://rosettacode.org/wiki/Host_introspection

fn main() {
    println!("word size: {} bits", 8 * std::mem::size_of::<usize>());

    if cfg!(target_endian = "big") {
        println!("big endian");
    } else {
        println!("little endian");
    }
}