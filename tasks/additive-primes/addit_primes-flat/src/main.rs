//===Flat implementation===

fn main() {
    let limit = 500;
    let column_w = limit.to_string().len() + 1;
    let mut pms = Vec::with_capacity(limit / 2 - limit / 3 / 2 - limit / 5 / 3 / 2 + 1);
    let mut count = 0;
    for u in (2..3).chain((3..limit).step_by(2)) {
        if pms.iter().take_while(|&&p| p * p <= u).all(|&p| u % p != 0) {
            pms.push(u);
            let dgs = std::iter::successors(Some(u), |&n| (n > 9).then(|| n / 10)).map(|n| n % 10);
            if pms.binary_search(&dgs.sum()).is_ok() {
                print!("{}{u:column_w$}", if count % 10 == 0 { "\n" } else { "" });
                count += 1;
            }
        }
    }
    println!("\n---\nFound {count} additive primes less than {limit}");
}
