use std::ops::Range;

fn greatest_subsequential_sum(nums: &[i32]) -> (i32, Range<usize>) {
    let mut max = 0;
    let mut boundaries = 0..0;

    for length in 0..nums.len() {
        for start in 0..nums.len() - length {
            let sum = (&nums[start..start + length]).iter().sum();
            if sum > max {
                max = sum;
                boundaries = start..start + length;
            }
        }
    }

    (max, boundaries)
}

fn main() {
    let nums = [1, 2, 39, 34, 20, -20, -16, 35, 0];

    let (max, boundaries) = greatest_subsequential_sum(&nums);

    println!("Max subsequence sum: {} for {:?}", max, &nums[boundaries]);;
}

#[test]
fn subsequential_sum() {
    let nums = [1, 2, 39, 34, 20, -20, -16, 35, 0];

    let (max, boundaries) = greatest_subsequential_sum(&nums);

    assert_eq!(max, 96);
    assert_eq!(&nums[boundaries], &[1, 2, 39, 34, 20]);
}
