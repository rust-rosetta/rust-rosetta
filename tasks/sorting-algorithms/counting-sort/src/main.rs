#[cfg_attr(feature="clippy", allow(needless_range_loop))]
fn counting_sort(array: &mut [i32], min: i32, max: i32) {
    // nothing to do for arrays shorter than 2
    if array.len() < 2 {
        return;
    }

    // we count occurences of values
    let size = (max - min + 1) as usize;
    let mut count = vec![0; size];

    for e in array.iter() {
        count[(*e - min) as usize] += 1;
    }

    // then we write values back, sorted
    let mut index = 0;
    for value in 0..count.len() {
        for _ in 0..count[value] {
            array[index] = value as i32;
            index += 1;
        }
    }
}

fn main() {
    let mut numbers = [4i32, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    counting_sort(&mut numbers, -31, 782);
}

#[cfg(test)]
mod tests {
    extern crate meta;

    fn check_sort(array: &mut [i32], min: i32, max: i32) {
        super::counting_sort(array, min, max);

        meta::test_utils::check_sorted(array);
    }

    #[test]
    fn rosetta_vector() {
        let numbers = &mut [4i32, 65, 2, -31, 0, 99, 2, 83, 782, 1];
        check_sort(numbers, -31, 782);
    }

    #[test]
    fn one_element_vector() {
        let numbers = &mut [0i32];
        check_sort(numbers, 0, 0);
    }

    #[test]
    fn repeat_vector() {
        let numbers = &mut [1i32, 1, 1, 1, 1];
        check_sort(numbers, 1, 1);
    }

    #[test]
    fn worst_case_vector() {
        let numbers = &mut [20i32, 10, 0, -1, -5];
        check_sort(numbers, -5, 20);
    }

    #[test]
    fn already_sorted_vector() {
        let numbers = &mut [-1i32, 0, 3, 6, 99];
        check_sort(numbers, -1, 99);
    }

    #[test]
    #[should_panic]
    fn bad_min() {
        let numbers = &mut [-1i32, 0, 3, 6, 99];
        check_sort(numbers, 2, 99);
    }
}
