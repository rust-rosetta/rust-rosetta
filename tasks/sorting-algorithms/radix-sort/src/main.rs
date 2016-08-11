fn merge(in1: Vec<i32>, in2: Vec<i32>, out: &mut [i32]) {
    let (left, right) = out.split_at_mut(in1.len());
    left.clone_from_slice(in1.as_slice());
    right.clone_from_slice(in2.as_slice());
}

// least significant digit radix sort
fn radix_sort(data: &mut [i32]) {
    for bit in 0..31 {
        // types of small and big is Vec<i32>.
        // It will be infered from the next call of merge function.
        let (small, big) = data.iter().partition(|&x| ((*x >> bit) & 1) == 0);
        merge(small, big, data);
    }
    // last bit is sign
    let (negative, positive) = data.iter().partition(|&x| *x < 0);
    merge(negative, positive, data);
}

fn main() {
    let mut data = [170, 45, 75, -90, -802, 24, 2, 66, -17, 2];
    println!("Before: {:?}", data);
    radix_sort(&mut data);
    println!("After: {:?}", data);
}

#[cfg(test)]
mod tests {
    extern crate rand;
    use self::rand::{thread_rng, Rng};

    use super::radix_sort;

    fn check_numbers(numbers: &mut [i32]) {
        let mut correct = numbers.to_vec();
        correct.sort();
        radix_sort(numbers);
        assert_eq!(correct, numbers);
    }

    #[test]
    fn test_rosetta_vector() {
        check_numbers(&mut [170, 45, 75, -90, -802, 24, 2, 66, -17, 2]);
    }

    #[test]
    fn test_empty_vector() {
        check_numbers(&mut []);
    }

    #[test]
    fn test_one_element_vector() {
        check_numbers(&mut [0i32]);
    }

    #[test]
    fn test_repeat_vector() {
        check_numbers(&mut [1i32, 1, 1, 1, 1]);
    }

    #[test]
    fn test_already_sorted_vector() {
        check_numbers(&mut [-1i32, 0, 3, 6, 99]);
    }

    #[test]
    fn test_random_numbers() {
        let mut rng = thread_rng();
        let mut numbers: Vec<i32> = rng.gen_iter::<i32>().take(500).collect();
        check_numbers(numbers.as_mut_slice());
    }
}
