const MAX_N: u64 = 250;

#[cfg_attr(feature = "clippy", allow(needless_range_loop))]
fn eulers_sum_of_powers() -> (usize, usize, usize, usize, usize) {
    let pow5: Vec<u64> = (0..MAX_N).map(|i| i.pow(5)).collect();
    let pow5_to_n = |pow| pow5.binary_search(&pow);

    for x0 in 1..MAX_N as usize {
        for x1 in 1..x0 {
            for x2 in 1..x1 {
                for x3 in 1..x2 {
                    let pow_sum = pow5[x0] + pow5[x1] + pow5[x2] + pow5[x3];
                    if let Ok(n) = pow5_to_n(pow_sum) {
                        return (x0, x1, x2, x3, n);
                    }
                }
            }
        }
    }

    panic!();
}

fn main() {
    let (x0, x1, x2, x3, y) = eulers_sum_of_powers();
    println!("{}^5 + {}^5 + {}^5 + {}^5 == {}^5", x0, x1, x2, x3, y)
}
