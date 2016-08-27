fn is_self_describing(mut n: u64) -> bool {
    // Compute the length of the number (the number of digits)
    let mut tmp = n;
    let mut len = 0;
    while tmp > 0 {
        len += 1;
        tmp /= 10;
    }

    // In base 10, numbers with more than 10 digits can't be self-describing
    if len > 10 || len == 0 {
        return false;
    }

    // Go through each digit of the number, count how many times each digit occurs, and then
    // subtract how often each digit is supposed to occur according to the number
    let mut cnt = [0i32; 10];
    for i in 0..len {
        cnt[(n % 10) as usize] += 1;
        cnt[len - i - 1] -= (n % 10) as i32;
        n /= 10;
    }

    // If the number is self-describing, then all counters should be zero
    cnt.iter().all(|&c| c == 0)
}

fn main() {
    // Print out all self-describing numbers below 10^8
    for i in 0u64..100_000_000 {
        if is_self_describing(i) {
            println!("{} is self-describing", i);
        }
    }
}

#[test]
fn test_is_self_describing() {
    let tests = [(0, false),
                 (1, false),
                 (200, false),
                 (1337, false),
                 (2020, true),
                 (1210, true),
                 (21200, true),
                 (3211000, true),
                 (42101000, true),
                 (43101000, false),
                 (521001000, true),
                 (6210001000, true)];

    for &(n, expected) in &tests {
        assert_eq!(is_self_describing(n), expected);
    }
}
