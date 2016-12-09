/// The mean is not defined for an empty list, so we must return an Option
fn rms(list: &[f64]) -> Option<f64> {
    match list.len() {
        0 => None,
        n => {
            let sum = list.iter().fold(0., |a, &b| a + b * b);
            Some(f64::sqrt(sum / n as f64))
        }
    }
}

fn main() {
    let input: Vec<_> = (1..11).map(|x| x as f64).collect();

    // 6.2048368229954285
    let rms = rms(&input).unwrap();
    println!("{}", rms);
}

#[test]
fn test() {
    let arr = [5., 7., 2., 0., 0.5];
    assert_eq!(rms(&arr), Some(3.9560080889704965));
}

#[test]
fn rms_empty_array() {
    let no_nums = [];
    assert_eq!(rms(&no_nums), None);
}
