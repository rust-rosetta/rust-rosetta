// http://rosettacode.org/wiki/Sorting_algorithms/Comb_sort

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
    extern crate rust_rosetta;

    use std::fmt::Debug;

    fn check_sort<T>(v: &mut [T])
        where T: Ord + Clone + Debug
    {
        super::comb_sort(v);
        rust_rosetta::check_sorted(v);
    }

    #[test]
    fn rosetta_vector() {
        let numbers = &mut [4i32, 65, 2, -31, 0, 99, 2, 83, 782, 1];
        check_sort(numbers);
    }

    #[test]
    fn empty_vector() {
        let mut numbers: &mut [i32] = &mut [];
        check_sort(numbers);
    }

    #[test]
    fn one_element_vector() {
        let numbers = &mut [0i32];
        check_sort(numbers);
    }

    #[test]
    fn repeat_vector() {
        let numbers = &mut [1i32, 1, 1, 1, 1];
        check_sort(numbers);
    }

    #[test]
    fn already_sorted_vector() {
        let numbers = &mut [-1i32, 0, 3, 6, 99];
        check_sort(numbers);
    }
}
