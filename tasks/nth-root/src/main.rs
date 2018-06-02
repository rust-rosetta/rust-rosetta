const EPSILON: f64 = 1e-10;

fn nth_root(x: f64, n: f64) -> f64 {
    let mut x0: f64 = x;
    loop {
        let delta = (x / x0.powf(n - 1.0) - x0) / n;
        x0 += delta;
        if delta.abs() < EPSILON {
            return x0;
        }
    }
}

fn main() {
    println!("{}", nth_root(8.0, 3.0));
    println!("{}", nth_root(4.0, 2.0));
    println!("{}", nth_root(169.0, 2.0));
    println!("{}", nth_root(81.0, 4.0));
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use self::rand::distributions::Uniform;
    use self::rand::Rng;
    use super::{nth_root, EPSILON};

    #[test]
    fn check_nth_root() {
        let mut rng = rand::thread_rng();
        let range = Uniform::new(1.0, 1e5);
        for _ in 1..1000 {
            let x = rng.sample(range);
            let n = rng.sample(range);
            assert!((nth_root(x, n) - x.powf(n.recip())).abs() < EPSILON)
        }
    }
}
