use std::cmp::{max, min};

static TENS: [u64; 20] = [
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
    1000000000000,
    10000000000000,
    100000000000000,
    1000000000000000,
    10000000000000000,
    100000000000000000,
    1000000000000000000,
    10000000000000000000,
];

/// Get the number of digits present in x
fn ndigits(mut x: u64) -> u64 {
    let mut n = 0;

    while x != 0 {
        n += 1;
        x /= 10;
    }

    n
}

fn dtally(mut x: u64) -> u64 {
    let mut t = 0;

    while x != 0 {
        t += 1 << ((x % 10) * 6);
        x /= 10;
    }

    t
}

/// Get a list of all fangs of x. Get only the first divider of each fang. The second one can be found simply with x / fang.
fn fangs(x: u64) -> Vec<u64> {
    let mut nd = ndigits(x) as usize;

    let mut fangs = vec![];

    if nd & 1 != 1 {
        nd /= 2;

        let lo = max(TENS[nd - 1], (x + TENS[nd] - 2) / (TENS[nd] - 1));
        let hi = min(x / lo, (x as f64).sqrt() as u64);

        let t = dtally(x);

        for a in lo..(hi + 1) {
            let b = x / a;
            if a * b == x && ((a % 10) > 0 || b % 10 > 0) && t == dtally(a) + dtally(b) {
                fangs.push(a);
            }
        }
    }

    fangs
}

/// Pretty print the fangs of x
fn print_fangs(x: u64, fangs: Vec<u64>) {
    print!("{} = ", x);

    if fangs.is_empty() {
        print!("is not vampiric");
    } else {
        for fang in fangs {
            print!("{} x {}, ", fang, x / fang);
        }
    }
    print!("\n");
}

fn main() {
    println!("The first 25 vampire numbers are :");

    let mut nfangs = 0;
    let mut x = 1;

    while nfangs < 25 {
        let fangs = fangs(x);
        if !fangs.is_empty() {
            nfangs += 1;
            print_fangs(x, fangs);
        }

        x += 1;
    }

    println!("\nSpecial requests :");

    print_fangs(16758243290880, fangs(16758243290880));
    print_fangs(24959017348650, fangs(24959017348650));
    print_fangs(14593825548650, fangs(14593825548650));
}

#[test]
fn test() {
    assert_eq!(
        fangs(16758243290880),
        vec![1982736, 2123856, 2751840, 2817360]
    );

    assert_eq!(
        fangs(24959017348650),
        vec![2947050, 2949705, 4125870, 4129587, 4230765]
    );

    assert_eq!(fangs(14593825548650), vec![]);
}
