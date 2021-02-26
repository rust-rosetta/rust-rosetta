fn same_digits(x: u64, base: u64) -> bool {
    let f = x % base;
    let mut n = x;
    while n > 0 {
        if n % base != f {
            return false;
        }
        n /= base;
    }

    true
}
fn is_brazilian(x: u64) -> bool {
    if x < 7 {
        return false;
    };
    if x % 2 == 0 {
        return true;
    };

    for base in 2..(x - 1) {
        if same_digits(x, base) {
            return true;
        }
    }
    false
}

fn main() {
    let mut counter = 0;
    let limit = 20;
    let big_limit = 100_000;
    let mut big_result: u64 = 0;
    let mut br: Vec<u64> = Vec::new();
    let mut o: Vec<u64> = Vec::new();
    let mut p: Vec<u64> = Vec::new();

    for x in 7.. {
        if is_brazilian(x) {
            counter += 1;
            if br.len() < limit {
                br.push(x);
            }
            if o.len() < limit && x % 2 == 1 {
                o.push(x);
            }
            if p.len() < limit && primes::is_prime(x) {
                p.push(x);
            }
            if counter == big_limit {
                big_result = x;
                break;
            }
        }
    }
    println!("First {} Brazilian numbers:", limit);
    println!("{:?}", br);
    println!("\nFirst {} odd Brazilian numbers:", limit);
    println!("{:?}", o);
    println!("\nFirst {} prime Brazilian numbers:", limit);
    println!("{:?}", p);

    println!("\nThe {}th Brazilian number: {}", big_limit, big_result);
}

#[cfg(test)]
mod tests {
    use super::is_brazilian;

    #[test]
    fn test_is_brazilian() {
        assert_eq!(false, is_brazilian(0));
        assert_eq!(true, is_brazilian(7));
        assert_eq!(true, is_brazilian(8));
        assert_eq!(false, is_brazilian(11));
        assert_eq!(true, is_brazilian(2801));
    }
}
