const EPSILON: f64 = 1e-10;

fn nth_root(x: f64, n: f64) -> f64{
    let mut x0: f64 = x;
    loop {
        let delta = (x/x0.powf(n-1.) - x0)/n;
        x0 += delta;
        if delta.abs() < EPSILON {
            return x0;
        }
    }
}

fn main() {
    println!("{}", nth_root(8., 3.));
    println!("{}", nth_root(4., 2.));
    println!("{}", nth_root(169., 2.));
    println!("{}", nth_root(81., 4.));
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use self::rand::Rng;
    use super::{nth_root, EPSILON};

    #[test]
    fn check_nth_root() {
        let mut rng = rand::thread_rng();
        for _ in 1..1000 {
            let x = rng.gen_range(1., 1e5);
            let n = rng.gen_range(1., 1e3);
            assert!((nth_root(x, n) - x.powf(1. / n)).abs() < EPSILON)
        }
    }
}
