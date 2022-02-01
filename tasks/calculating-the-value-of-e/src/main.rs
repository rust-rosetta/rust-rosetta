const EPSILON: f64 = 1e-15;

fn main() {
    let mut fact: u64 = 1;
    let mut e: f64 = 2.0;
    let mut n: u64 = 2;
    loop {
        let e0 = e;
        fact *= n;
        n += 1;
        e += 1.0 / fact as f64;
        if (e - e0).abs() < EPSILON {
            break;
        }
    }

    println!("e = {:.15}", e);
}
