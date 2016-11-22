/// implementation based on observation:
/// `josephus(n, k) == if n < 2 { 0 } else { (josephus(n - 1, k) + k) % n }`
fn josephus(n: u32, k: u32) -> u32 {
    let mut result = 0u32;
    for i in 2..(n + 1) {
        result = (result + k) % i;
    }
    result
}

fn main() {
    let n = 5;
    let k = 2;
    println!("n: {} k: {} survivor: {}", n, k, josephus(n, k));
}

#[cfg(test)]
mod tests {
    use super::josephus;

    #[test]
    fn test() {
        assert!(josephus(5, 2) == 2);
        assert!(josephus(41, 3) == 30);
    }
}
