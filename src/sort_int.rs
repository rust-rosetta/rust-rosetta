// Implements http://rosettacode.org/wiki/Sort_an_integer_array

#[cfg(not(test))]
fn main() {
    let mut a = vec!(9, 8, 7, 6, 5, 4, 3, 2, 1, 0);

    // Merge sort in place, allocates ~2*n memory
    a.sort();
    println!("{}", a);
}
