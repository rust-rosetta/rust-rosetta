use std::cmp::min;

fn getfill(pattern: &[usize]) -> usize {
    let mut total = 0;
    for (idx, val) in pattern.iter().enumerate() {
        let l_peak = pattern[..idx].iter().max();
        let r_peak = pattern[idx + 1..].iter().max();
        if l_peak.is_some() && r_peak.is_some() {
            let peak = min(l_peak.unwrap(), r_peak.unwrap());
            if peak > val {
                total += peak - val;
            }
        }
    }
    total
}

fn main() {
    let patterns = vec![
        vec![1, 5, 3, 7, 2],
        vec![5, 3, 7, 2, 6, 4, 5, 9, 1, 2],
        vec![2, 6, 3, 5, 2, 8, 1, 4, 2, 2, 5, 3, 5, 7, 4, 1],
        vec![5, 5, 5, 5],
        vec![5, 6, 7, 8],
        vec![8, 7, 7, 6],
        vec![6, 7, 10, 7, 6],
    ];

    for pattern in patterns {
        println!("pattern: {:?}, fill: {}", &pattern, getfill(&pattern));
    }
}

#[test]
fn test_getfill() {
    let patterns = vec![
        vec![1, 5, 3, 7, 2],
        vec![5, 3, 7, 2, 6, 4, 5, 9, 1, 2],
        vec![2, 6, 3, 5, 2, 8, 1, 4, 2, 2, 5, 3, 5, 7, 4, 1],
        vec![5, 5, 5, 5],
        vec![5, 6, 7, 8],
        vec![8, 7, 7, 6],
        vec![6, 7, 10, 7, 6],
    ];

    let answers = vec![2, 14, 35, 0, 0, 0, 0];

    for (idx, pattern) in patterns.iter().enumerate() {
        assert_eq!(answers[idx], getfill(&pattern));
    }
}
