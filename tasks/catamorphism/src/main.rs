fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let sum = numbers.iter().fold(0, |a, n| a + n);
    println!("{}", sum);
    let product = numbers.iter().fold(1, |a, n| a * n);
    println!("{}", product);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sum() {
        let sum = [1, 2, 3, 4, 5].iter().fold(0, |a, n| a + n);
        assert_eq!(sum, 15);
    }
    #[test]
    fn test_product() {
        let product = [1, 2, 3, 4, 5].iter().fold(1, |a, n| a * n);
        assert_eq!(product, 120);
    }
}
