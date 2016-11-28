fn main() {
    println!("new vec filtered: ");
    let nums: Vec<i32> = (1..20).collect();
    let evens: Vec<i32> = nums.iter().cloned().filter(|x| x % 2 == 0).collect();
    println!("{:?}", evens);

    // Filter an already existing vector
    println!("original vec filtered: ");
    let mut nums: Vec<i32> = (1..20).collect();
    nums.retain(|x| x % 2 == 0);
    println!("{:?}", nums);
}
