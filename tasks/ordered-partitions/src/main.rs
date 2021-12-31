use itertools::Itertools;

type NArray = Vec<Vec<Vec<usize>>>;

fn generate_partitions(args: &[usize]) -> NArray {
    // calculate the sum of all partitions
    let max = args.iter().sum();

    // generate combinations with the given lengths
    // for each partition
    let c = args.iter().fold(vec![], |mut acc, arg| {
        acc.push((1..=max).combinations(*arg).collect::<Vec<_>>());
        acc
    });

    // create a cartesian product of all individual combinations
    // filter/keep only where all the elements are there and exactly once
    c.iter()
        .map(|i| i.iter().cloned())
        .multi_cartesian_product()
        .unique()
        .filter(|x| x.iter().cloned().flatten().unique().count() == max)
        .collect::<Vec<_>>()
}

#[allow(clippy::ptr_arg)]
fn print_partitions(result: &NArray) {
    println!("Partitions:");
    for partition in result {
        println!("{:?}", partition);
    }
}
fn main() {
    print_partitions(generate_partitions(&[2, 0, 2]).as_ref());
    print_partitions(generate_partitions(&[1, 1, 1]).as_ref());
    print_partitions(generate_partitions(&[2, 3]).as_ref());
    print_partitions(generate_partitions(&[0]).as_ref());
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_partitions() {
        itertools::assert_equal(generate_partitions(&[0]), vec![vec![vec![]]]);
        itertools::assert_equal(
            generate_partitions(&[2, 0, 2]),
            vec![
                vec![vec![1, 2], vec![], vec![3, 4]],
                vec![vec![1, 3], vec![], vec![2, 4]],
                vec![vec![1, 4], vec![], vec![2, 3]],
                vec![vec![2, 3], vec![], vec![1, 4]],
                vec![vec![2, 4], vec![], vec![1, 3]],
                vec![vec![3, 4], vec![], vec![1, 2]],
            ],
        );

        itertools::assert_equal(
            generate_partitions(&[1, 1, 1]),
            vec![
                vec![vec![1], vec![2], vec![3]],
                vec![vec![1], vec![3], vec![2]],
                vec![vec![2], vec![1], vec![3]],
                vec![vec![2], vec![3], vec![1]],
                vec![vec![3], vec![1], vec![2]],
                vec![vec![3], vec![2], vec![1]],
            ],
        );
    }
}
