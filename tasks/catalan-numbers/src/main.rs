fn c_n(n: u64) -> u64 {
    match n {
        0 => 1,
        _ => c_n(n - 1) * 2 * (2 * n - 1) / (n + 1),
    }
}

fn main() {
    for i in 1..16 {
        println!("c_n({}) = {}", i, c_n(i));
    }
}
