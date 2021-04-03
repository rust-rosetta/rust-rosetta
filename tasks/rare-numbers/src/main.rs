use itertools::Itertools;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;
use std::time::Instant;

#[derive(Debug)]
struct RareResults {
    digits: u8,
    time_to_find: u128,
    counter: u32,
    number: u64,
}

impl fmt::Display for RareResults {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:>6} {:>6} ms {:>2}. {}",
            self.digits, self.time_to_find, self.counter, self.number
        )
    }
}

fn print_results(results: Vec<RareResults>) {
    println!("Results:");
    println!("digits      time  #. Rare number");
    for r in results {
        println!("{}", r);
    }
}

fn isqrt(n: u64) -> u64 {
    let mut s = (n as f64).sqrt() as u64;
    s = (s + n / s) >> 1;
    if s * s > n {
        s - 1
    } else {
        s
    }
}

fn is_square(n: u64) -> bool {
    match n & 0xf {
        0 | 1 | 4 | 9 => {
            let t = isqrt(n);
            t * t == n
        }
        _ => false,
    }
}

fn get_reverse(number: &u64) -> u64 {
    number
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .fold(0_u64, |a, (i, d)| a + 10_u64.pow(i as u32) * d as u64)
}
fn is_rare(number: u64) -> bool {
    let reverse = get_reverse(&number);

    reverse != number
        && number > reverse
        && is_square(number + reverse)
        && is_square(number - reverse)
}

/// This method is a very simple naive search, using brute-force to check a high amount of numbers
/// for satisfying the rare number criterias. As such it is rather slow, and above 10 digits it's
/// not really performant, release version takes ~30 secs to find the first 5 (max 10 digits)
fn naive(digits: u8) {
    let bp_equal = (0_u8..=9).zip(0_u8..=9).collect::<Vec<(u8, u8)>>();
    let bp_zero_or_even = (0_u8..=9)
        .cartesian_product(0_u8..=9)
        .filter(|pair| (pair.0 == pair.1) || (pair.0 as i32 - pair.1 as i32).abs() % 2 == 0)
        .collect::<Vec<(u8, u8)>>();

    let bp_odd = (0_u8..=9)
        .cartesian_product(0_u8..=9)
        .filter(|pair| (pair.0 as i32 - pair.1 as i32).abs() % 2 == 1)
        .collect::<Vec<(u8, u8)>>();

    let bp_9 = (0_u8..=9)
        .cartesian_product(0_u8..=9)
        .filter(|pair| pair.0 + pair.1 == 9)
        .collect::<Vec<(u8, u8)>>();

    let bp_73 = (0_u8..=9)
        .cartesian_product(0_u8..=9)
        .filter(|pair| [7, 3].contains(&(pair.0 as i8 - pair.1 as i8)))
        .collect::<Vec<(u8, u8)>>();

    let bp_11 = (0_u8..=9)
        .cartesian_product(0_u8..=9)
        .filter(|pair| pair.0 + pair.1 == 11 || pair.1 + pair.0 == 1)
        .collect::<Vec<(u8, u8)>>();

    let aq_bp_setup: Vec<((u8, u8), &Vec<(u8, u8)>)> = vec![
        ((2, 2), &bp_equal),
        ((4, 0), &bp_zero_or_even),
        ((6, 0), &bp_odd),
        ((6, 5), &bp_odd),
        ((8, 2), &bp_9),
        ((8, 3), &bp_73),
        ((8, 7), &bp_11),
        ((8, 8), &bp_equal),
    ];

    //generate AB-PQ combinations
    let aq_bp = aq_bp_setup
        .iter()
        .map(|e| {
            e.1.iter().fold(vec![], |mut out, b| {
                out.push(vec![e.0 .0, b.0, b.1, e.0 .1]);
                out
            })
        })
        .flatten()
        .collect::<Vec<_>>();

    let mut results: Vec<RareResults> = Vec::new();
    let mut counter = 0_u32;
    let start_time = Instant::now();

    for d in 1..=digits {
        print!("Digits: {} ", d);

        if d < 4 {
            for n in 10_u64.pow((d - 1).into())..10_u64.pow(d.into()) {
                if is_rare(n) {
                    counter += 1;
                    results.push(RareResults {
                        digits: d,
                        time_to_find: start_time.elapsed().as_millis(),
                        counter,
                        number: n,
                    });
                }
            }
        } else {
            aq_bp.iter().for_each(|abqp| {
                let start = abqp[0] as u64 * 10_u64.pow((d - 1).into())
                    + abqp[1] as u64 * 10_u64.pow((d - 2).into())
                    + 10_u64 * abqp[2] as u64
                    + abqp[3] as u64;

                // brute-force checking all numbers which matches the pattern AB...PQ
                // very slow
                for n in (start..start + 10_u64.pow((d - 2).into())).step_by(100) {
                    if is_rare(n) {
                        counter += 1;
                        results.push(RareResults {
                            digits: d,
                            time_to_find: start_time.elapsed().as_millis(),
                            counter,
                            number: n,
                        });
                    }
                }
            });
        }

        println!(
            "Digits: {} done - Elapsed time(ms): {}",
            d,
            start_time.elapsed().as_millis()
        );
    }

    print_results(results);
}

/// This algorithm uses an advanced search strategy based on Nigel Galloway's approach,
/// and can find the first 40 rare numers (16 digits) within reasonable
/// time in release version
fn advanced(digits: u8) {
    // setup
    let mut results: Vec<RareResults> = Vec::new();
    let mut counter = 0_u32;
    let start_time = Instant::now();

    let numeric_digits = (0..=9).map(|x| [x, 0]).collect::<Vec<_>>();
    let diffs1: Vec<i8> = vec![0, 1, 4, 5, 6];

    // all possible digits pairs to calaculate potential diffs
    let pairs = (0_i8..=9)
        .cartesian_product(0_i8..=9)
        .map(|x| [x.0, x.1])
        .collect::<Vec<_>>();
    let all_diffs = (-9i8..=9).collect::<Vec<_>>();

    // lookup table for the first diff
    let lookup_1 = vec![
        vec![[2, 2], [8, 8]], //Diff = 0
        vec![[8, 7], [6, 5]], //Diff = 1
        vec![],
        vec![],
        vec![[4, 0]],         // Diff = 4
        vec![[8, 3]],         // Diff = 5
        vec![[6, 0], [8, 2]], // Diff = 6
    ];

    // lookup table for all the remaining diffs
    let lookup_n: HashMap<i8, Vec<_>> = pairs.into_iter().into_group_map_by(|elt| elt[0] - elt[1]);

    for d in 2..=digits {
        // powers like 1, 10, 100, 1000....
        let powers = (0..d).map(|x| 10_u64.pow(x.into())).collect::<Vec<u64>>();

        // for n-r (aka L) the required terms, like 9/ 99 / 999 & 90 / 99999 & 9999 & 900 etc
        let terms = powers
            .iter()
            .zip(powers.iter().rev())
            .map(|(a, b)| b.checked_sub(*a).unwrap_or(0))
            .filter(|x| *x != 0)
            .collect::<Vec<u64>>();

        // create a cartesian product for all potetential diff numbers
        // for the first use the very short one, for all other the complete 19 element
        let diff_list_iter = (0_u8..(d / 2))
            .map(|i| match i {
                0 => diffs1.iter(),
                _ => all_diffs.iter(),
            })
            .multi_cartesian_product()
            // remove invalid first diff/second diff combinations - custom iterator would be probably better
            .filter(|x| {
                if x.len() == 1 {
                    return true;
                }
                match (*x[0], *x[1]) {
                    (a, b) if (a == 0 && b != 0) => false,
                    (a, b) if (a == 1 && ![-7, -5, -3, -1, 1, 3, 5, 7].contains(&b)) => false,
                    (a, b) if (a == 4 && ![-8, -6, -4, -2, 0, 2, 4, 6, 8].contains(&b)) => false,
                    (a, b) if (a == 5 && ![7, -3].contains(&b)) => false,
                    (a, b) if (a == 6 && ![-9 - 7, -5, -3, -1, 1, 3, 5, 7, 9].contains(&b)) => {
                        false
                    }
                    _ => true,
                }
            });

        #[cfg(debug_assertions)]
        {
            println!("  powers: {:?}", powers);
            println!("  terms: {:?}", terms);
        }

        diff_list_iter.for_each(|diffs| {
            // calculate difference of original n and its reverse (aka L = n-r)
            // which must be a perfect square
            let l: i64 = diffs
                .iter()
                .zip(terms.iter())
                .map(|(diff, term)| **diff as i64 * *term as i64)
                .sum();

            if l > 0 && is_square(l.try_into().unwrap()) {
                // potential candiate, at least L is a perfect square
                #[cfg(debug_assertions)]
                println!("  square L: {}, diffs: {:?}", l, diffs);

                // placeholder for the digits
                let mut dig: Vec<i8> = vec![0_i8; d.into()];

                // generate a cartesian product for each identified diff using the lookup tables
                let c_iter = (0..(diffs.len() + d as usize % 2))
                    .map(|i| match i {
                        0 => lookup_1[*diffs[0] as usize].iter(),
                        _ if i != diffs.len() => lookup_n.get(diffs[i]).unwrap().iter(),
                        _ => numeric_digits.iter(), // for the middle digits
                    })
                    .multi_cartesian_product();

                // check each H (n+r) by using digit combination
                c_iter.for_each(|elt| {
                    // print!("    digits combinations: {:?}", elt);
                    for (i, digit_pair) in elt.iter().enumerate() {
                        // print!("  digit pairs: {:?}, len: {}", digit_pair, l.len());
                        dig[i] = digit_pair[0];
                        dig[d as usize - 1 - i] = digit_pair[1]
                    }

                    // for numbers with odd # digits restore the middle digit
                    // which has been overwritten at the end of the previous cycle
                    if d % 2 == 1 {
                        dig[(d as usize - 1) / 2] = elt[elt.len() - 1][0];
                    }

                    let num = dig
                        .iter()
                        .rev()
                        .enumerate()
                        .fold(0_u64, |acc, (i, d)| acc + 10_u64.pow(i as u32) * *d as u64);

                    let reverse = dig
                        .iter()
                        .enumerate()
                        .fold(0_u64, |acc, (i, d)| acc + 10_u64.pow(i as u32) * *d as u64);

                    if num > reverse && is_square(num + reverse) {
                        println!("  FOUND: {}, reverse: {}", num, reverse);
                        counter += 1;
                        results.push(RareResults {
                            digits: d,
                            time_to_find: start_time.elapsed().as_millis(),
                            counter,
                            number: num,
                        });
                    }
                });
            }
        });

        println!(
            "Digits: {} done - Elapsed time(ms): {}",
            d,
            start_time.elapsed().as_millis()
        );
    }

    print_results(results);
}
fn main() {
    println!("Run this program in release mode for measuring performance");
    println!("Naive version:");
    naive(10);

    println!("Advanced version:");
    advanced(15);
}
