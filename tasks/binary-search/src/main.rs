fn main() {
    println!("{:?}", binary_search(&[1, 2, 3, 4, 5, 6], 4));
    println!("{:?}", binary_search_rec(&[1, 2, 3, 4, 5, 6], 4));
}

/// iterative version
fn binary_search<T: Ord>(haystack: &[T], needle: T) -> Option<usize> {
    let mut low = 0;
    let mut high = haystack.len() - 1;

    if high == 0 {
        return None;
    }

    while low <= high {
        // avoid overflow
        let mid = (low + high) >> 1;

        if haystack[mid] > needle {
            high = mid - 1
        } else if haystack[mid] < needle {
            low = mid + 1
        } else {
            return Some(mid);
        }
    }
    None
}

/// recursive version
fn binary_search_rec<T: Ord>(haystack: &[T], needle: T) -> Option<usize> {
    fn recurse<T: Ord>(low: usize, high: usize, haystack: &[T], needle: T) -> Option<usize> {
        match (low + high) / 2 {
            _ if high < low => None,
            mid if haystack[mid] > needle => recurse(low, mid - 1, haystack, needle),
            mid if haystack[mid] < needle => recurse(mid + 1, high, haystack, needle),
            mid => Some(mid),
        }
    }
    recurse::<T>(0, haystack.len() - 1, haystack, needle)
}

#[test]
fn test_result() {
    let haystack = &[1, 2, 3, 4, 5, 6];
    let needle = 4;

    assert_eq!(binary_search(haystack, needle), Some(3));
    assert_eq!(binary_search_rec(haystack, needle), Some(3));
}
