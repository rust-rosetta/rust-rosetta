fn a(x: bool) -> bool {
    println!("Inside function a");
    x
}

fn b(x: bool) -> bool {
    println!("Inside function b");
    x
}

fn main() {
    let booleans = [true, false];

    for &i in &booleans {
        for &j in &booleans {
            println!("{} and {} is {}", i, j, a(i) && b(j));
            println!("{} or {} is {}", i, j, a(i) || b(j));
        }
    }
}
