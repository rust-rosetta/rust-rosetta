fn sum(arr: &[f64]) -> f64 {
    return arr.iter().fold(0.0, |p,q| p + *q);
}

fn mean(arr: &[f64]) -> f64 {
    return sum(arr) / arr.len() as f64;
}

fn main() {
    let v = &[2.0, 3.0, 5.0, 7.0, 13.0, 21.0, 33.0, 54.0];
    println!("mean of {}: {}", v, mean(v));

    let w = &[];
    println!("mean of {}: {}", w, mean(w));
}
