fn main() {
    let arr: [i32; 9] = [1i32, 2, 3, 4, 5, 6, 7, 8, 9];

    // using fold
    let sum = arr.iter().fold(0i32, |a, &b| a + b);
    let product = arr.iter().fold(1i32, |a, &b| a * b);
    println!("the sum is {} and the product is {}", sum, product);

    // or using sum and product
    let sum = arr.iter().sum::<i32>();
    let product = arr.iter().product::<i32>();
    println!("the sum is {} and the product is {}", sum, product);
}
