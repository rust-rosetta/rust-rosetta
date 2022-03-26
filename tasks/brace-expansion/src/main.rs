use brace_expansion::{expand, tokenize};

fn main() {
    let mut input: String = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let tokens = tokenize(&input);
    let expanded = expand(tokens);

    for line in &expanded {
        println!("{}", line);
    }
}
