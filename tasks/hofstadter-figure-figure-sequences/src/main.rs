use std::collections::HashMap;

struct Hffs {
    sequence_r: HashMap<usize, usize>,
    sequence_s: HashMap<usize, usize>,
}

impl Hffs {
    fn new() -> Hffs {
        Hffs {
            sequence_r: HashMap::new(),
            sequence_s: HashMap::new(),
        }
    }
    fn ffr(&mut self, n: usize) -> usize {
        // first try the cache
        let new_r = if let Some(result) = self.sequence_r.get(&n) {
            *result
        } else if n == 0 {
            1
        } else {
            // call recursively
            self.ffr(n - 1) + self.ffs(n - 1)
        };

        // insert into the cache and return value
        *self.sequence_r.entry(n).or_insert(new_r)
    }

    fn ffs(&mut self, n: usize) -> usize {
        // first try the cache
        let new_s = if let Some(result) = self.sequence_s.get(&n) {
            *result
        } else if n == 0 {
            2
        } else {
            let lower = self.ffs(n - 1) + 1_usize;
            let upper = self.ffr(n) + 1_usize;
            let mut min_s: usize = 0;
            // find next available S
            for i in lower..=upper {
                if !self.sequence_r.values().any(|&val| val == i) {
                    min_s = i;
                    break;
                }
            }
            min_s
        };

        // insert into the cache and return value
        *self.sequence_s.entry(n).or_insert(new_s)
    }
}

impl Default for Hffs {
    fn default() -> Self {
        Self::new()
    }
}
fn main() {
    let mut hof = Hffs::new();

    for i in 0..10 {
        println!("H:{} -> R: {}, S: {}", i, hof.ffr(i), hof.ffs(i));
    }

    let r40 = (0..40).map(|i| hof.ffr(i)).collect::<Vec<_>>();
    let mut s960 = (0..960).map(|i| hof.ffs(i)).collect::<Vec<_>>();

    s960.extend(&r40);
    s960.sort_unstable();
    let f1000 = (1_usize..=1000).collect::<Vec<_>>();

    assert_eq!(f1000, s960, "Does NOT match");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hof() {
        let mut hof = Hffs::new();

        let f10 = (0..10).map(|i| hof.ffr(i)).collect::<Vec<_>>();
        assert_eq!(f10, vec![1, 3, 7, 12, 18, 26, 35, 45, 56, 69]);
    }
}
