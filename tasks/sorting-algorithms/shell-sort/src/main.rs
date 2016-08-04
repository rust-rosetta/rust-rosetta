#[cfg(test)]
#[macro_use]
extern crate meta;

#[cfg_attr(feature="clippy", allow(needless_range_loop))]
fn shell_sort<T: Ord + Copy>(v: &mut [T]) {
    let mut gap = v.len() / 2;
    let len = v.len();
    while gap > 0 {
        for i in gap..len {
            let temp = v[i];
            let mut j = i;
            while j >= gap && v[j - gap] > temp {
                v[j] = v[j - gap];
                j -= gap;
            }
            v[j] = temp;
        }
        gap /= 2;
    }
}

fn main() {
    let mut numbers = [4i32, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);
    shell_sort(&mut numbers);
    println!("After: {:?}", numbers);
}

#[cfg(test)]
mod tests {
    test_sort!(super::shell_sort);
}
