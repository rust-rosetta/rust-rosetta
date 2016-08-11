#[cfg(test)]
#[macro_use]
extern crate meta;

// Progress through the slice and 'bubble' elements up and down until they are in order.
#[cfg_attr(feature = "clippy", allow(needless_range_loop))]
fn cocktail_sort<T: PartialOrd>(v: &mut [T]) {
    (1..v.len() + 1).rev().all(|length| {
        let mut swapped: bool = false;
        // bubble up
        for index in 0..length - 1 {
            if v[index] > v[index + 1] {
                swapped = true;
                v.swap(index, index + 1);
            }
        }
        // break if no swap occured before bubbling down
        if !swapped {
            return false;
        }
        // bubble down
        for index in (0..length - 1).rev() {
            if v[index] > v[index + 1] {
                swapped = true;
                v.swap(index, index + 1);
            }
        }
        // Continue to iterate if any swapping took place
        swapped
    });
}

fn main() {
    let mut numbers = [4i32, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);
    cocktail_sort(&mut numbers);
    println!("After: {:?}", numbers);
}

#[cfg(test)]
mod tests {
    test_sort!(super::cocktail_sort);
}
