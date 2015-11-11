// http://rosettacode.org/wiki/Loops/N_plus_one_half


fn main() {
    for i in 1..11 {
        print!("{}", i);
        if i == 10 {
            break;
        }
        print!(", ");
    }
}
