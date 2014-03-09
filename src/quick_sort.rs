//Implements http://rosettacode.org/wiki/Sorting_algorithms/Quicksort
fn quick_sort<T: Ord + Clone>(v: &mut[T]) {
    let vLen = v.len();
    if vLen < 2 {
        return;
    }
    let mut leftIndex = 0;
    let mut rightIndex = vLen-1;
    let pivot: T = v[vLen/2].clone();
    while leftIndex <= rightIndex {
        if v[leftIndex] < pivot {
            leftIndex = leftIndex+1;
            continue;
        }
        if v[rightIndex] > pivot {
            rightIndex = rightIndex-1;
            continue;
        }
        v.swap(leftIndex, rightIndex);
        leftIndex = leftIndex+1;
        rightIndex = rightIndex-1;
    }
   
    {
        let leftArray = v.mut_slice(0, rightIndex+1);
        quick_sort(leftArray);
    }
    {
        let rightArray = v.mut_slice(leftIndex, vLen);
        quick_sort(rightArray);
    }
}

fn main() {
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    quick_sort(numbers);
}

#[cfg(test)]
fn check_sort<T: Ord>(v: &[T]) {
    if v.len() > 1 {
        for i in range(0, v.len()-1) {
            assert!(v[i] <= v[i+1]);
        }
    }
}

#[test]
fn test_rosetta_vector() {
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    quick_sort(numbers);
    check_sort(numbers);
}

#[test]
fn test_empty_vector() {
    let mut numbers: ~[int] = ~[];
    quick_sort(numbers);
    check_sort(numbers);
}

#[test]
fn test_one_element_vector() {
    let mut numbers = [0];
    quick_sort(numbers);
    check_sort(numbers);
}

#[test]
fn test_repeat_vector() {
    let mut numbers = [1, 1, 1, 1, 1];
    quick_sort(numbers);
    check_sort(numbers);
}

#[test]
fn test_worst_case_vector() {
    let mut numbers = [20, 10, 0, -1, -5];
    quick_sort(numbers);
    check_sort(numbers);
}

#[test]
fn test_already_sorted_vector() {
    let mut numbers = [-1, 0, 3, 6, 99];
    quick_sort(numbers);
    check_sort(numbers);
}
