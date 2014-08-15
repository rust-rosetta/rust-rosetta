// http://rosettacode.org/wiki/Self-describing_numbers

fn is_self_describing(mut n: u64) -> bool {
    let mut tmp = n;
    let mut len = 0u;
    while tmp > 0 {
        len += 1;
        tmp /= 10;
    }

    if len > 10 || len == 0 {
        return false;
    }

    let mut cnt = [0i, ..10];
    for i in range(0u, len) {
        cnt[(n % 10) as uint] += 1;
        cnt[len - i - 1] -= (n % 10) as int;
        n /= 10;
    }

    cnt.iter().all(|&c| c == 0)
}

#[cfg(not(test))]
fn main() {
    for i in range(0, 100_000_000) {
        if is_self_describing(i) {
            println!("{} is self-describing", i);
        }
    }
}

#[test]
fn test_is_self_describing() {
    let tests = [
        (0, false),
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
        (6210001000, true),
    ];

    for &(n, expected) in tests.iter() {
        assert_eq!(is_self_describing(n), expected);
    }
}

