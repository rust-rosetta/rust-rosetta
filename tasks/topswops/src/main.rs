use itertools::Itertools;

fn solve(deck: &[usize]) -> usize {
    let mut counter = 0_usize;
    let mut shuffle = deck.to_vec();
    loop {
        let p0 = shuffle[0];
        if p0 == 1 {
            break;
        }
        shuffle[..p0].reverse();
        counter += 1;
    }

    counter
}

// this is a naive method which tries all permutations and works up to ~12 cards
fn topswops(number: usize) -> usize {
    (1..=number)
        .permutations(number)
        .fold(0_usize, |mut acc, p| {
            let steps = solve(&p);
            if steps > acc {
                acc = steps;
            }
            acc
        })
}
fn main() {
    (1_usize..=10).for_each(|x| println!("{}: {}", x, topswops(x)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topswops() {
        assert_eq!(topswops(1), 0);
        assert_eq!(topswops(9), 30);
    }
}
