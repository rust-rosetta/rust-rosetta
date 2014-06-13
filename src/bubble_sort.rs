//Implements http://rosettacode.org/wiki/Sorting_algorithms/Bubble_sort
fn bubble_sort<T: PartialOrd>(v: &mut[T]) {
    let mut done = v.len()<1;
    let mut length = v.len();
    while !done {
        done = true;
        length = length-1;
        for index in range(0, length) {
            if v[index] > v[index+1] {
                done = false;
                v.swap(index, index+1);
            }
        }
    }
}

#[cfg(not(test))]
fn main() {
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    bubble_sort(numbers);
}

#[cfg(test)]
fn check_sort<T: PartialOrd>(v: &[T]) {
    if v.len() > 1 {
        for i in range(0, v.len()-1) {
            assert!(v[i] <= v[i+1]);
        }
    }
}

#[test]
fn test_rosetta_vector() {
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    bubble_sort(numbers);
    check_sort(numbers);
}

#[test]
fn test_empty_vector() {
    let numbers: &mut[int] = &mut[];
    bubble_sort(numbers);
    check_sort(numbers);
}

#[test]
fn test_one_element_vector() {
    let mut numbers = [0];
    bubble_sort(numbers);
    check_sort(numbers);
}

#[test]
fn test_repeat_vector() {
    let mut numbers = [1, 1, 1, 1, 1];
    bubble_sort(numbers);
    check_sort(numbers);
}

#[test]
fn test_worst_case_vector() {
    let mut numbers = [20, 10, 0, -1, -5];
    bubble_sort(numbers);
    check_sort(numbers);
}

#[test]
fn test_already_sorted_vector() {
    let mut numbers = [-1, 0, 3, 6, 99];
    bubble_sort(numbers);
    check_sort(numbers);
}
