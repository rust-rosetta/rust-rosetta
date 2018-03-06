#[cfg(test)]
#[macro_use]
extern crate meta;

fn pancake_sort<T: Ord>(v: &mut [T]) {
    let len = v.len();
    // trivial case -- no flips
    if len < 2 {
        return;
    }
    for i in (0..len).rev() {
        // find index of the maximum element within `v[0..i]` (inclusive)
        let max_index = v.iter()
            .take(i + 1)
            .enumerate()
            .max_by_key(|&(_, elem)| elem)
            .map(|(idx, _)| idx)
            // safe because we already checked if `v` is empty
            .unwrap();
        // if `max_index` is not where it's supposed to be
        // do two flips to move it to `i`
        if max_index != i {
            flip(v, max_index);
            flip(v, i);
        }
    }
}

// function to flip a section of a mutable collection from 0..num (inclusive)
fn flip<E: PartialOrd>(v: &mut [E], num: usize) {
    v[0..num + 1].reverse();
}

fn main() {
    // Sort numbers
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);
    pancake_sort(&mut numbers);
    println!("After: {:?}", numbers);

    // Sort strings
    let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
    println!("Before: {:?}", strings);
    pancake_sort(&mut strings);
    println!("After: {:?}", strings);
}

#[cfg(test)]
mod tests {
    test_sort!(super::pancake_sort);
}
