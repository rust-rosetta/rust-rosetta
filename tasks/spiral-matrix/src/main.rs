const VECTORS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub fn spiral_matrix(size: usize) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0; size]; size];
    let mut movement = VECTORS.iter().cycle();
    let (mut x, mut y, mut n) = (-1, 0, 0..);

    for (move_x, move_y) in std::iter::once(size)
        .chain((1..size).rev().flat_map(|n| std::iter::repeat(n).take(2)))
        .flat_map(|steps| std::iter::repeat(movement.next().unwrap()).take(steps))
    {
        x += move_x;
        y += move_y;
        matrix[y as usize][x as usize] = n.next().unwrap();
    }

    matrix
}

fn main() {
    for i in spiral_matrix(5).iter() {
        for j in i.iter() {
            print!("{:>2} ", j);
        }
        println!();
    }
}

#[test]
fn empty_spiral() {
    let expected: Vec<Vec<u32>> = Vec::new();
    assert_eq!(spiral_matrix(0), expected);
}

#[test]
fn size_one_spiral() {
    let expected: Vec<Vec<u32>> = vec![vec![0]];
    assert_eq!(spiral_matrix(1), expected);
}
#[test]
fn size_two_spiral() {
    let expected: Vec<Vec<u32>> = vec![vec![0, 1], vec![3, 2]];
    assert_eq!(spiral_matrix(2), expected);
}

#[test]
fn size_three_spiral() {
    let expected: Vec<Vec<u32>> = vec![vec![0, 1, 2], vec![7, 8, 3], vec![6, 5, 4]];
    assert_eq!(spiral_matrix(3), expected);
}
#[test]
fn size_four_spiral() {
    let expected: Vec<Vec<u32>> = vec![
        vec![0, 1, 2, 3],
        vec![11, 12, 13, 4],
        vec![10, 15, 14, 5],
        vec![9, 8, 7, 6],
    ];
    assert_eq!(spiral_matrix(4), expected);
}
#[test]
fn size_five_spiral() {
    let expected: Vec<Vec<u32>> = vec![
        vec![0, 1, 2, 3, 4],
        vec![15, 16, 17, 18, 5],
        vec![14, 23, 24, 19, 6],
        vec![13, 22, 21, 20, 7],
        vec![12, 11, 10, 9, 8],
    ];
    assert_eq!(spiral_matrix(5), expected);
}
