fn main() {
    for i in 1..6 {
        for _ in 1..i + 1 {
            print!("*")
        }
        println!("")
    }
}
