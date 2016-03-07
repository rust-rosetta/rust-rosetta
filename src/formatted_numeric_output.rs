// http://rosettacode.org/wiki/Formatted_numeric_output

fn main() {
    let x = 7.125;

    println!("{:9}", x);
    println!("{:09}", x);
    println!("{:9}", -x);
    println!("{:09}", -x);
}
