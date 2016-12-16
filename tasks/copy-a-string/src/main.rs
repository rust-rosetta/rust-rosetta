fn main() {
    let s1 = "A String";

    // Create an additional reference to "A String".
    let s2: &str = s1;

    // Create a copy of "A String"
    let s3: String = s1.to_string();

    println!("s1 = {}, s2 = {}, s3 = {}", s1, s2, s3);
}
