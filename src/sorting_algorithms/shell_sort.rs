// http://rosettacode.org/wiki/Sorting_algorithms/Shell_sort

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
    extern crate rust_rosetta;

    use std::fmt::Debug;

    fn check_sort<T>(v: &mut [T])
        where T: Ord + Copy + Clone + Debug
    {
        super::shell_sort(v);

        rust_rosetta::check_sorted(v);
    }

    #[test]
    fn test_rosetta_vector() {
        let numbers = &mut [4i32, 65, 2, -31, 0, 99, 2, 83, 782, 1];
        check_sort(numbers);
    }

    #[test]
    fn test_empty_vector() {
        let mut numbers: Vec<i32> = Vec::new();
        check_sort(&mut numbers[..]);
    }

    #[test]
    fn test_one_element_vector() {
        let numbers = &mut [0i32];
        check_sort(numbers);
    }

    #[test]
    fn test_repeat_vector() {
        let numbers = &mut [1i32, 1, 1, 1, 1];
        check_sort(numbers);
    }

    #[test]
    fn test_string_vector() {
        let strings = &mut ["beach", "hotel", "airplane", "car", "house", "art"];
        check_sort(strings);
    }
}
