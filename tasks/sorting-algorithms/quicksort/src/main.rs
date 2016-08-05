#[cfg(test)]
#[macro_use]
extern crate meta;

/// We use an [in-place quick sort].
///
/// [in-place quick sort]: http://en.wikipedia.org/wiki/Quicksort#In-place_version
fn quick_sort<T: Ord>(v: &mut [T]) {
    let len = v.len();
    if len < 2 {
        return;
    }

    let pivot_index = partition(v);

    // Sort the left side
    quick_sort(&mut v[0..pivot_index]);

    // Sort the right side
    quick_sort(&mut v[pivot_index + 1..len]);
}

/// Reorders the slice with values lower than the pivot at the left side,
/// and values bigger than it at the right side.
/// Also returns the store index.
#[cfg_attr(feature="clippy", allow(needless_range_loop))]
fn partition<T: Ord>(v: &mut [T]) -> usize {
    let len = v.len();
    let pivot_index = len / 2;

    v.swap(pivot_index, len - 1);

    let mut store_index = 0;
    for i in 0..(len - 1) {
        if v[i] <= v[len - 1] {
            v.swap(i, store_index);
            store_index += 1;
        }
    }

    v.swap(store_index, len - 1);
    store_index
}

fn main() {
    // Sort numbers
    let mut numbers = [4i32, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);

    quick_sort(&mut numbers);
    println!("After: {:?}", numbers);

    // Sort strings
    let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
    println!("Before: {:?}", strings);

    quick_sort(&mut strings);
    println!("After: {:?}", strings);
}

#[cfg(test)]
mod tests {
    test_sort!(super::quick_sort);
}
