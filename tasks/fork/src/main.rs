use std::process::Command;

fn main() {
    println!("parent");

    let _ = Command::new("echo")
        .arg("child")
        .spawn()
        .unwrap();
}
