use decimal::d128;
use factorial::Factorial;

fn hickerson(n: u64) -> d128 {
    d128::from(n.factorial()) / (d128!(2) * (d128!(2).ln().pow(d128::from(n + 1))))
}

// Some details on floating-points numbers can be found at https://cheats.rs/#basic-types
fn main() {
    for i in 1..18 {
        let h = hickerson(i);
        let string = h.to_string();
        let dec_part = string.split('.').nth(1).unwrap();
        if dec_part.starts_with('0') || dec_part.starts_with('9') {
            println!("{} is an almost integer.", h);
        } else {
            println!("{} is not an almost integer.", h);
        }
    }
}
