fn multi_hello() -> (&'static str, i32) {
    ("Hello", 42)
}

fn main() {
    let (string, num) = multi_hello();
    println!("{}, {}", string, num);
}
