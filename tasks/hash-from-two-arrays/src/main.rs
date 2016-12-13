use std::collections::HashMap;

fn main() {
    let keys = ["a", "b", "c"];
    let values = [1, 2, 3];

    let hash = keys.iter().zip(values.iter()).collect::<HashMap<_, _>>();
    println!("{:?}", hash);
}
