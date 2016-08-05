#[cfg(test)]
#[macro_use]
extern crate meta;

#[cfg_attr(feature="clippy", allow(needless_range_loop))]
fn selection_sort<T: Ord>(v: &mut [T]) {
    if v.is_empty() {
        return;
    }

    let len = v.len();

    for j in 0..len - 1 {
        let mut min_index = j;
        for i in j + 1..len {
            if v[i] <= v[min_index] {
                min_index = i;
            }
        }
        v.swap(j, min_index);
    }
}

fn main() {
    // Sort numbers
    let mut numbers = [4i32, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);

    selection_sort(&mut numbers);
    println!("After: {:?}", numbers);

    // Sort strings
    let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
    println!("Before: {:?}", strings);

    selection_sort(&mut strings);
    println!("After: {:?}", strings);
}

#[cfg(test)]
mod tests {
    test_sort!(super::selection_sort);
}
