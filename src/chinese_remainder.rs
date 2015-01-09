// http://rosettacode.org/wiki/Chinese_remainder_theorem

#[cfg(not(test))]
fn main() {
    let l = [(2, 3), (3, 5), (2, 7)];
    println!("{:?}", chinese_remainder(&l));
}

fn chinese_remainder(l: &[(int, int)]) -> Option<int> {
    let mut product = 1;
    for &(_, n) in l.iter() {
        product *= n;
    }

    let mut sum = 0;
    for &(a, n) in l.iter() {
        let mut term = product / n;

        let inv = match mul_inv(term, n) {
            Some(inv) => inv,
            None => return None
        };

        term *= inv;
        term *= a;
        sum += term;
    }
    Some(sum % product)
}

fn mul_inv(a: int, b: int) -> Option<int> {
    let (gcd, mut x, _) = egcd(a, b);
    if gcd != 1 { // No multiplicative inverse exists
        return None;
    }
    if x < 0 {
        x += b;
    }
    Some(x % b)
}

fn egcd(a: int, b: int) -> (int, int, int) {
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
