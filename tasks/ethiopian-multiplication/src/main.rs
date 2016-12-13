fn double(a: i32) -> i32 {
    2 * a
}

fn halve(a: i32) -> i32 {
    a / 2
}

fn is_even(a: i32) -> bool {
    a % 2 == 0
}

fn ethiopian_multiplication(mut x: i32, mut y: i32) -> i32 {
    let mut sum = 0;

    while x >= 1 {
        print!("{} \t {}", x, y);
        if is_even(x) {
            println!("\t Not Kept");
        } else {
            println!("\t Kept");
            sum += y;
        }
        x = halve(x);
        y = double(y);
    }
    sum
}

fn main() {
    let output = ethiopian_multiplication(17, 34);
    println!("---------------------------------");
    println!("\t {}", output);
}
