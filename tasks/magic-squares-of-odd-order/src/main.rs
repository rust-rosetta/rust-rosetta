fn main() {
    let n = 9;
    let mut square = vec![vec![0; n]; n];
    for (i, row) in square.iter_mut().enumerate() {
        for (j, e) in row.iter_mut().enumerate() {
            *e = n * (((i + 1) + (j + 1) - 1 + (n >> 1)) % n) +
                 (((i + 1) + (2 * (j + 1)) - 2) % n) + 1;
            print!("{:3} ", e);
        }
        println!("");
    }
    let sum = n * (((n * n) + 1) / 2);
    println!("The sum of the square is {}.", sum);
}
