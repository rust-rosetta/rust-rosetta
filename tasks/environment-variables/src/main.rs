use std::env;

fn main() {
    let variables = ["PATH", "HOME", "USER"];

    for variable in &variables {
        match env::var(variable) {
            Ok(value) => println!("{}={}", variable, value),
            Err(e) => println!("Could not read {}: {}.", variable, e),
        }
    }
}
