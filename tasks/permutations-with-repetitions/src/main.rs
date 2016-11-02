struct PermutationIterator<'a, T: 'a> {
    universe: &'a [T],
    size: usize,
    prev: Option<Vec<usize>>,
}

fn permutations<T>(universe: &[T], size: usize) -> PermutationIterator<T> {
    PermutationIterator {
        universe: universe,
        size: size,
        prev: None,
    }
}

fn map<T>(values: &[T], ixs: &[usize]) -> Vec<T>
    where T: Clone
{
    ixs.iter().map(|&i| values[i].clone()).collect()
}

impl<'a, T> Iterator for PermutationIterator<'a, T>
    where T: Clone
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {

        let n = self.universe.len();

        if n == 0 {
            return None;
        }

        match self.prev {
            None => {
                let zeroes: Vec<usize> = std::iter::repeat(0)
                    .take(self.size)
                    .collect();
                let result = Some(map(self.universe, &zeroes[..]));
                self.prev = Some(zeroes);
                result
            }
            Some(ref mut indexes) => {
                match indexes.iter().position(|&i| i + 1 < n) {
                    None => None,
                    Some(position) => {
                        for index in indexes.iter_mut().take(position) {
                            *index = 0;
                        }
                        indexes[position] += 1;
                        Some(map(self.universe, &indexes[..]))
                    }
                }
            }
        }
    }
}

fn main() {
    let universe = ["Annie", "Barbie"];
    for p in permutations(&universe[..], 3) {
        for element in &p {
            print!("{} ", element);
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::permutations;

    #[test]
    fn test_len() {
        let universe = [1, 132, 323];
        assert!(permutations(&universe[..], 4).count() == 81);
    }
}
