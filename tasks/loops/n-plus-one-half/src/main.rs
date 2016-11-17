fn main() {
    for i in 1..11 {
        print!("{}", i);
        if i == 10 {
            break;
        }
        print!(", ");
    }
}
