fn main() {
    // Similar to C++, Rust offers raw strings:
    let x = r#"
        This is a "raw string literal," roughly equivalent to a heredoc.
    "#;

    println!("{}", x);
}
