/// Search for the last occurence of term if v. If term is not found in v, 0 is returned.
fn search_term(v: &[u64], term: &u64) -> u64 {
    if v.is_empty() {
        0
    } else {
        let (last, rest) = v.split_last().unwrap();
        if last == term {
            1
        } else {
            let term = search_term(rest, term);
            if term != 0 {
                term + 1
            } else {
                term
            }
        }
    }
}

/// Get a Van Eck sequence of size n
fn van_eck(n: usize) -> Vec<u64> {
    if n == 1 {
        vec![0]
    } else {
        let mut prev = van_eck(n - 1);

        let (term, v) = prev.split_last().unwrap();
        let new = search_term(v, term);

        prev.push(new);
        prev
    }
}

fn main() {
    println!("The first 10 terms : {:?}", van_eck(10));
    println!(
        "The 991 to 1000 terms : {:?}",
        van_eck(1000).split_at(990).1
    );
}

#[test]
fn test() {
    assert_eq!(van_eck(10), [0, 0, 1, 0, 2, 0, 2, 2, 1, 6]);
    assert_eq!(
        van_eck(1000).split_at(990).1,
        [4, 7, 30, 25, 67, 225, 488, 0, 10, 136]
    );
}
