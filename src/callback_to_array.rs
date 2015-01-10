// Implements http://rosettacode.org/wiki/Apply_a_callback_to_an_array

fn main () {
    let array = [1,2,3,4,5];
    println!("{:?}", array);

    println!("{:?}", array.iter()
             // The map does not modify the original array.
             // It just returns a 'lazy' iterator.
             .map(callback)
             // To get a result, we 'consume' the iterator by
             // collecting it into a `Vec`.
             .collect::<Vec<i32>>());
}

fn callback(val: &i32) -> i32 {
    *val + 1
}
