#![feature(inclusive_range_syntax)]

use std::cmp;
use std::collections::BTreeMap;
use std::i32;
use std::ops::{Index, IndexMut};

/// A 2-D array for storing data per-vertex pair. 1-indexed.
#[derive(Debug, Default)]
struct VertexArray<T> {
    dimension: usize,
    data: Vec<T>,
}

impl<T> VertexArray<T>
where
    T: Default + Copy + Clone,
{
    fn new(dimension: usize) -> Self {
        Self {
            dimension,
            data: vec![T::default(); dimension * dimension],
        }
    }
}

impl<T> Index<(usize, usize)> for VertexArray<T> {
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        let (i, j) = (i - 1, j - 1);
        &self.data[self.dimension * i + j]
    }
}

impl<T> IndexMut<(usize, usize)> for VertexArray<T> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        let (i, j) = (i - 1, j - 1);
        &mut self.data[self.dimension * i + j]
    }
}

/// Returns a `BTreeMap` mapping pairs of vertices to a pair consisting of the weight of the
/// shortest path between them, and a `Vec` containing the path itself.
fn floyd_warshall(graph: &[((usize, usize), i32)]) -> BTreeMap<(usize, usize), (i32, Vec<usize>)> {
    let num_vertices = graph
        .iter()
        .map(|&((u, v), _)| cmp::max(u, v))
        .max()
        .unwrap();

    let mut dist = VertexArray::<Option<i32>>::new(num_vertices);
    let mut next = VertexArray::<Option<usize>>::new(num_vertices);

    for &((u, v), w) in graph {
        dist[(u, v)] = Some(w);
        next[(u, v)] = Some(v);
    }

    for k in 1..=num_vertices {
        dist[(k, k)] = Some(0);
    }

    for k in 1..=num_vertices {
        for i in 1..=num_vertices {
            for j in 1..=num_vertices {
                if let (Some(w1), Some(w2)) = (dist[(i, k)], dist[(k, j)]) {
                    if dist[(i, j)].is_none() || dist[(i, j)].unwrap() > w1 + w2 {
                        dist[(i, j)] = Some(w1 + w2);
                        next[(i, j)] = next[(i, k)];
                    }
                }
            }
        }
    }

    let mut pairs = BTreeMap::new();

    for i in 1..=num_vertices {
        for j in 1..=num_vertices {
            if i == j {
                continue;
            }

            let distance = dist[(i, j)].unwrap();
            let path = path(&next, (i, j));
            pairs.insert((i, j), (distance, path));
        }
    }

    pairs
}

fn path(next: &VertexArray<Option<usize>>, (mut u, v): (usize, usize)) -> Vec<usize> {
    let mut path = vec![u];

    while u != v && next[(u, v)].is_some() {
        u = next[(u, v)].unwrap();
        path.push(u);
    }

    path
}

fn main() {
    let weights = vec![
        ((1, 3), -2),
        ((2, 1), 4),
        ((2, 3), 3),
        ((3, 4), 2),
        ((4, 2), -1),
    ];

    println!("{:<7} {:<7} {}", "pair", "dist", "path");
    let pairs = floyd_warshall(&weights);
    for (pair, &(distance, ref path)) in &pairs {
        println!(
            "{:<7} {:<7} {:?}",
            format!("{} -> {}", pair.0, pair.1),
            format!("{:>4}", distance),
            path
        );
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    #[test]
    fn floyd_warshall() {
        let weights = vec![
            ((1, 3), -2),
            ((2, 1), 4),
            ((2, 3), 3),
            ((3, 4), 2),
            ((4, 2), -1),
        ];

        let mut expected = BTreeMap::new();
        expected.insert((1, 2), (-1, vec![1, 3, 4, 2]));
        expected.insert((1, 3), (-2, vec![1, 3]));
        expected.insert((1, 4), (0, vec![1, 3, 4]));
        expected.insert((2, 1), (4, vec![2, 1]));
        expected.insert((2, 3), (2, vec![2, 1, 3]));
        expected.insert((2, 4), (4, vec![2, 1, 3, 4]));
        expected.insert((3, 1), (5, vec![3, 4, 2, 1]));
        expected.insert((3, 2), (1, vec![3, 4, 2]));
        expected.insert((3, 4), (2, vec![3, 4]));
        expected.insert((4, 1), (3, vec![4, 2, 1]));
        expected.insert((4, 2), (-1, vec![4, 2]));
        expected.insert((4, 3), (1, vec![4, 2, 1, 3]));

        assert_eq!(expected, super::floyd_warshall(&weights));
    }
}
