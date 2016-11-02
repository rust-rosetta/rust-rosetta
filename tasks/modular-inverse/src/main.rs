fn main() {
    println!("{:?}", mul_inv(42, 2017));
}

fn mul_inv(a: i32, b: i32) -> Option<i32> {
    let (gcd, mut x, _) = egcd(a, b);
    if gcd != 1 {
        // No multiplicative inverse exists
        return None;
    }
    if x < 0 {
        x += b;
    }
    Some(x % b)
}

#[cfg_attr(feature = "clippy", allow(many_single_char_names))]
fn egcd(a: i32, b: i32) -> (i32, i32, i32) {
    if a == 0 {
        return (b, 0, 1);
    }

    let (g, y, x) = egcd(b % a, a);
    (g, x - (b / a) * y, y)
}

#[test]
fn test() {
    assert_eq!(mul_inv(42, 2017), Some(1969));
}
