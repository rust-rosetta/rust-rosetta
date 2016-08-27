fn sq_sum(v: &[f64]) -> f64 {
    v.iter().fold(0., |sum, &num| sum + num * num)
}

fn main() {
    let v = vec![3.0, 1.0, 4.0, 1.0, 5.5, 9.7];
    println!("{}", sq_sum(&v));

    let u: Vec<f64> = vec![];
    println!("{}", sq_sum(&u));
}
