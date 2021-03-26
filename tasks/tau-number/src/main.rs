/// Gets all divisors of a number, including itself
fn get_divisors(n: u32) -> Vec<u32> {
    let mut results = Vec::new();

    for i in 1..(n / 2 + 1) {
        if n % i == 0 {
            results.push(i);
        }
    }
    results.push(n);
    results
}

fn is_tau_number(i: u32) -> bool {
    0 == i % get_divisors(i).len() as u32
}

fn main() {
    println!("\nFirst 100 Tau numbers:");
    let mut counter: u32 = 0;
    let mut i: u32 = 1;
    while counter < 100 {
        if is_tau_number(i) {
            print!("{:>4}", i);
            counter += 1;
            print!("{}", if counter % 20 == 0 { "\n" } else { "," });
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::is_tau_number;

    #[test]
    fn test_is_tau_number() {
        assert_eq!(is_tau_number(1), true);
        assert_eq!(is_tau_number(3), false);
        assert_eq!(is_tau_number(1096), true);
    }
}
