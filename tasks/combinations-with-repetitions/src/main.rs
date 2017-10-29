// Iterator for the combinations of `arr` with `k` elements with repetitions.
// Yields the combinations in lexicographical order.
struct CombinationsWithRepetitions<'a, T: 'a> {
    // source array to get combinations from
    arr: &'a Vec<T>,
    // length of the combinations
    k: u32,
    // current counts of each object that represent the next combination
    counts: Vec<u32>,
    // whether there are any combinations left
    remaining: bool
}

impl<'a, T> CombinationsWithRepetitions<'a, T> {
    fn new(arr: &Vec<T>, k: u32) -> CombinationsWithRepetitions<T> {
        let mut counts = vec![0; arr.len()];
        counts[arr.len() - 1] = k;
        let comb = CombinationsWithRepetitions { arr: arr, k: k, counts: counts, remaining: true };
        comb
    }
}

impl<'a, T> Iterator for CombinationsWithRepetitions<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Vec<&'a T>> {
        if !self.remaining {
            return None
        }
        let mut comb = Vec::new();
        for (count, item) in self.counts.iter().zip(self.arr.iter()) {
            for _ in 0..count.clone() {
                comb.push(item.clone());
            }
        }
        // this is lexicographically largest, and thus the last combination
        if self.counts[0] == self.k {
            self.remaining = false;
        } else {
            let n = self.counts.len();
            for i in (1..n).rev() {
                if self.counts[i] > 0 {
                    let original_value = self.counts[i];
                    self.counts[i-1] += 1;
                    for j in i..(n - 1) {
                        self.counts[j] = 0;
                    }
                    self.counts[n - 1] = original_value-1;
                    break;
                }
            }
        }
        Some(comb)
    }
}


fn main() {
    let collection = vec!["iced", "jam", "plain"];
    for comb in CombinationsWithRepetitions::new(&collection, 2) {
        for item in comb.iter() {
            print!("{} ", item)
        }
        println!()
    }
}
