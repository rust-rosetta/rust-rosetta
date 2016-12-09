fn main() {
    let array = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    println!("{:?}",
             array.iter()
                 .map(callback)
                 .collect::<Vec<i32>>());
}

fn callback(val: &i32) -> i32 {
    *val + 1
}
