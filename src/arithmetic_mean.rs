// Implements http://rosettacode.org/wiki/Averages/Arithmetic_mean

fn mean(list: &[f64]) -> Option<f64> {
    match list.len() {
        0 => None,
        n => {
            let sum = list.iter().fold(0 as f64, |acc, &x| acc + x);
            Some(sum / n as f64)
        }
    }
}

fn main() {
    let input = vec!(3.0, 1.0, 4.0, 1.0, 5.0, 9.0);

    // This should be 3.833333
    let mean = mean(input.as_slice()).unwrap();
    println!("{}", mean);
}

#[test]
fn simple_test() {
    let vector = vec!(1.0, 2.0, 3.0, 4.0, 5.0);
    assert!(mean(vector.as_slice()).unwrap() == 3.0);
}

#[test]
fn mean_empty_list() {
    let empty_vec = vec!();
    assert!(mean(empty_vec.as_slice()).is_none());
}