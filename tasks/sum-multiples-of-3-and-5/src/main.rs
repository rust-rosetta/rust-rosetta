fn sum35(lim: u64) -> u64 {
    let mut sum: u64 = 0;
    for i in 3..lim {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    return sum;
}


fn main() {
    let limit = 10000;
    println!("Sum of multiples of 3 and 5 from 0 till {} are: {}",
             limit,
             sum35(limit));
}

#[test]
fn test_sum35() {
    assert_eq!(233168, sum35(1000));
    assert_eq!(35553600, sum35(12345));
}
