fn main() {
    for i in 1..10 + 1 {
        print!("{}", i);
        if i % 5 == 0 {
            print!("\n");
            continue;
        }
        print!(", ");
    }
}
