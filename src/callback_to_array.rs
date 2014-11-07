// Implements http://rosettacode.org/wiki/Apply_a_callback_to_an_array

fn main () {
    let array = [1,2,3,4,5];
    println!("{}", array.as_slice());

    println!("{}", array.iter()
                        // The map does not modify the original array.
                        // It just returns a 'lazy' iterator.
                        .map(callback)
                        // To get a result, we 'consume' the iterator by
                        // collecting it into a `Vec`.
                        .collect::<Vec<int>>());
}

fn callback(val: &int) -> int {
    *val + 1
}
