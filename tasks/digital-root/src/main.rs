
fn sum_digits(mut n: u64, base: u64) -> u64 {
    let mut sum = 0u64;
    while n > 0 {
        sum = sum + (n % base);
        n = n / base;
    }
    sum
}

// Returns tuple of (additive-persistence, digital-root)
fn digital_root(mut num: u64, base: u64) -> (u64, u64) {
    let mut pers = 0;
    while num >= base {
        pers = pers + 1;
        num = sum_digits(num, base);
    }
    (pers, num)
}

fn main() {

    // Test base 10
    let values = [627615u64, 39390u64, 588225u64, 393900588225u64];
    for &value in values.iter() {
        let (pers, root) = digital_root(value, 10);
        println!("{} has digital root {} and additive persistance {}",
                 value,
                 root,
                 pers);
    }

    println!("");

    // Test base 16
    let values_base16 = [0x7e0, 0x14e344, 0xd60141, 0x12343210];
    for &value in values_base16.iter() {
        let (pers, root) = digital_root(value, 16);
        println!("0x{:x} has digital root 0x{:x} and additive persistance 0x{:x}",
                 value,
                 root,
                 pers);
    }
}

#[test]
fn test_sum_digits() {
    let sum = sum_digits(15u64, 10u64);
    assert_eq!(sum, 6);
}

#[test]
fn test_base_ten() {

    // Test tuples: (value, root, pers)
    let test_pairs =
        [(627615u64, 9, 2), (39390u64, 6, 2), (588225u64, 3, 2), (393900588225u64, 9, 2)];

    for &(value, result_root, result_pers) in test_pairs.iter() {
        let (pers, root) = digital_root(value, 10u64);
        println!("{} has digital root {} and additive persistance {}",
                 value,
                 root,
                 pers);
        assert_eq!(pers, result_pers);
        assert_eq!(root, result_root);
    }
}

#[test]
fn test_base_16() {

    // Test tuples: (value, root, pers)
    let test_pairs =
        [(0x7e0, 0x6, 0x2), (0x14e344, 0xf, 0x2), (0xd60141, 0xa, 0x2), (0x12343210, 0x1, 0x2)];

    for &(value, result_root, result_pers) in test_pairs.iter() {
        let (pers, root) = digital_root(value, 16u64);
        println!("{:x} has digital root {:x} and additive persistance {:x}",
                 value,
                 root,
                 pers);
        assert_eq!(pers, result_pers);
        assert_eq!(root, result_root);
    }
}
