fn main() {
    let a = [1u, 2, 3, 4, 5];
    let x = a.iter().max().unwrap();
    println!("{}", x);
}
