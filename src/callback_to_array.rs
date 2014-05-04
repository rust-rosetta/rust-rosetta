// Implements http://rosettacode.org/wiki/Apply_a_callback_to_an_array

#[cfg(not(test))]
fn main () {
    let array = [1,2,3,4,5];

    println!("{}", array.as_slice());

    // The map does not mofify the array. It just returns an iterator.
    // We iterate through the results of map and collect them into a vector.
    println!("{}", array.iter().map(callback).collect::<Vec<int>>());
}

#[cfg(not(test))]
fn callback(val: &int) -> int {
    val + 1
}
