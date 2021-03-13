use std::convert::TryInto;

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

/// Calculates whether the divisors can be partitioned into two disjoint
/// sets that sum to the same value
fn is_summable(x: i32, divisors: &[u32]) -> bool {
    if !divisors.is_empty() {
        if divisors.contains(&(x as u32)) {
            return true;
        } else if let Some((first, t)) = divisors.split_first() {
            return is_summable(x - *first as i32, &t) || is_summable(x, &t);
        }
    }
    false
}

/// Calculates whether the number is a Zumkeller number
/// Zumkeller numbers are the set of numbers whose divisors can be partitioned
/// into two disjoint sets that sum to the same value. Each sum must contain
/// divisor values that are not in the other sum, and all of the divisors must
/// be in one or the other.
fn is_zumkeller_number(number: u32) -> bool {
    if number % 18 == 6 || number % 18 == 12 {
        return true;
    }

    let div = get_divisors(number);
    let divisor_sum: u32 = div.iter().sum();
    if divisor_sum == 0 {
        return false;
    }
    if divisor_sum % 2 == 1 {
        return false;
    }

    // numbers where n is odd and the abundance is even are Zumkeller numbers
    let abundance = divisor_sum as i32 - 2 * number as i32;
    if number % 2 == 1 && abundance > 0 && abundance % 2 == 0 {
        return true;
    }

    let half = divisor_sum / 2;
    return div.contains(&half)
        || (div.iter().filter(|&&d| d < half).count() > 0
            && is_summable(half.try_into().unwrap(), &div));
}

fn main() {
    println!("\nFirst 220 Zumkeller numbers:");
    let mut counter: u32 = 0;
    let mut i: u32 = 0;
    while counter < 220 {
        if is_zumkeller_number(i) {
            print!("{:>3}", i);
            counter += 1;
            print!("{}", if counter % 20 == 0 { "\n" } else { "," });
        }
        i += 1;
    }

    println!("\nFirst 40 odd Zumkeller numbers:");
    let mut counter: u32 = 0;
    let mut i: u32 = 3;
    while counter < 40 {
        if is_zumkeller_number(i) {
            print!("{:>5}", i);
            counter += 1;
            print!("{}", if counter % 20 == 0 { "\n" } else { "," });
        }
        i += 2;
    }
}

#[cfg(test)]
mod tests {
    use super::is_zumkeller_number;

    #[test]
    fn test_is_zumkeller() {
        assert_eq!(is_zumkeller_number(0), false);
        assert_eq!(is_zumkeller_number(6), true);
        assert_eq!(is_zumkeller_number(20), true);
        assert_eq!(is_zumkeller_number(21), false);
        assert_eq!(is_zumkeller_number(198), true);
    }
}
