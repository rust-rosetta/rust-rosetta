#[cfg(test)]
#[macro_use]
extern crate meta;

#[cfg_attr(feature="clippy", allow(needless_range_loop))]
fn comb_sort<T: PartialOrd>(v: &mut [T]) {
    let len = v.len();
    let mut gap: usize = v.len();
    let mut swapped: bool = true;
    while gap > 1 || swapped {
        if gap > 1 {
            gap = (gap as f32 / 1.25) as usize;
        }
        swapped = false;
        for i in 0..len - gap {
            if v[i] > v[i + gap] {
                swapped = true;
                v.swap(i, i + gap);
            }
        }
    }
}

fn main() {
    let mut numbers = [4i32, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);
    comb_sort(&mut numbers);
    println!("After: {:?}", numbers);
}

#[cfg(test)]
mod tests {
    test_sort!(super::comb_sort);
}
