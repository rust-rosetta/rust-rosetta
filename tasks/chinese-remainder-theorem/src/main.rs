fn main() {
    let l = [(2, 3), (3, 5), (2, 7)];
    println!("{:?}", chinese_remainder(&l));
}

fn chinese_remainder(l: &[(i32, i32)]) -> Option<i32> {
    let product = l.iter().fold(1, |prod, &(_, n)| prod * n);

    let mut sum = 0;
    for &(a, n) in l {
        let mut term = product / n;

        let inv = match mul_inv(term, n) {
            Some(inv) => inv,
            None => return None,
        };

        term *= inv;
        term *= a;
        sum += term;
    }
    Some(sum % product)
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
    let l = [(2, 3), (3, 5), (2, 7)];
    assert_eq!(chinese_remainder(&l), Some(23));

    let l = [(10, 11), (4, 22), (9, 19)];
    assert_eq!(chinese_remainder(&l), None);
}
