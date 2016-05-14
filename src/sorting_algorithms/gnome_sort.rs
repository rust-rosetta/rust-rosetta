// http://rosettacode.org/wiki/Sorting_algorithms/Gnome_sort

fn gnome_sort<T: PartialOrd>(v: &mut [T]) {
    let len = v.len();
    let mut i: usize = 1;
    let mut j: usize = 2;
    while i < len {
        if v[i - 1] <= v[i] {
            i = j;
            j = j + 1;
        } else {
            v.swap(i - 1, i);
            i = i - 1;
            if i == 0 {
                i = j;
                j = j + 1;
            }
        }
    }
}

fn main() {
    let mut numbers = [4i32, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);
    gnome_sort(&mut numbers);
    println!("After: {:?}", numbers);
}

#[cfg(test)]
mod tests {
    extern crate rust_rosetta;

    use std::fmt::Debug;

    fn check_sort<T>(v: &mut [T])
        where T: Ord + Clone + Debug
    {
        super::gnome_sort(v);

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
