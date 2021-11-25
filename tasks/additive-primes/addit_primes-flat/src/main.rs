//===Flat implementation===

fn main() {
    let limit = 500;
    //all primes, starting from 3 (including non-additive), will be collected in pms
    //  it works ~1.5 times faster than the variant with fn is_prime(){...while...+=6...}
    let mut pms = Vec::with_capacity(limit / 2 - limit / 3 / 2 - limit / 5 / 3 / 2 + 1);
    let column_width = limit.to_string().len() + 1;
    print!("{:1$}", 2, column_width);
    let mut count = 1;
    for u in (3..limit).step_by(2) {
        if pms.iter().take_while(|&&p| p * p <= u).all(|&p| u % p != 0) {
            pms.push(u);
            //about the same speed as while...{...+=...%.../=...}, but without mut
            let sum_digits = std::iter::successors(Some(u), |&n| (n > 9).then(|| n / 10))
                .fold(0, |s, n| s + n % 10);
            if sum_digits == 2 || matches!(pms.binary_search(&sum_digits), Ok(_)) {
                if count % 10 == 0 {
                    println!();
                }
                print!("{:1$}", u, column_width);
                count += 1;
            }
        }
    }
    println!("\n---\nFound {} additive primes less than {}", count, limit);
}
