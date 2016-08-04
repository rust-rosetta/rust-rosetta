#[cfg(test)]
#[macro_use]
extern crate meta;

/// Progress through the slice and 'bubble' elements up until they are in order.
#[cfg_attr(feature = "clippy", allow(needless_range_loop))]
fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    (1..v.len() + 1).rev().all(|length| {
        let mut changes = 0;

        for index in 0..length - 1 {
            if v[index] > v[index + 1] {
                changes += 1;
                v.swap(index, index + 1);
            }
        }

        // Continue to iterate if any 'bubble-ing' took place
        changes > 0
    });
}

fn main() {
    let mut numbers = [4i32, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    bubble_sort(&mut numbers);
}

#[cfg(test)]
mod tests {
    test_sort!(super::bubble_sort);
}
