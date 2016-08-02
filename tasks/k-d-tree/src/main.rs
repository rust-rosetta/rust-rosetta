extern crate time;
extern crate rand;

use std::cmp::Ordering;
use std::ops::Sub;
use std::cmp::Ordering::Less;

use rand::Rng;
use time::get_time;

#[derive(Clone, PartialEq, Debug)]
struct Point {
    pub coords: Vec<f32>,
}

impl<'a, 'b> Sub<&'b Point> for &'a Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Point {
        assert_eq!(self.coords.len(), rhs.coords.len());
        Point { coords: self.coords.iter().zip(rhs.coords.iter()).map(|(x, &y)| *x - y).collect() }
    }
}

impl Point {
    fn norm_sq(&self) -> f32 {
        self.coords.iter().fold(0f32, |a, &b| a + b * b)
    }
}

struct KDTreeNode {
    point: Point,
    dim: usize,
    // Construction could become faster if we use an arena allocator,
    // but this is easier to use.
    left: Option<Box<KDTreeNode>>,
    right: Option<Box<KDTreeNode>>,
}

impl KDTreeNode {
    /// Create a new KDTreeNode around the `dim`th dimension.
    /// Alternatively, we could dynamically determine the dimension to
    /// split on by using the longest dimension.
    pub fn new(points: &mut [Point], dim: usize) -> KDTreeNode {
        let points_len = points.len();
        if points_len == 1 {
            return KDTreeNode {
                point: points[0].clone(),
                dim: dim,
                left: None,
                right: None,
            };
        }

        // Split around the median
        let pivot = quickselect_by(points,
                                   points_len / 2,
                                   &|a, b| a.coords[dim].partial_cmp(&b.coords[dim]).unwrap());

        let left = Some(Box::new(KDTreeNode::new(&mut points[0..points_len / 2],
                                                 (dim + 1) % pivot.coords.len())));
        let right = if points.len() >= 3 {
            Some(Box::new(KDTreeNode::new(&mut points[points_len / 2 + 1..points_len],
                                          (dim + 1) % pivot.coords.len())))
        } else {
            None
        };

        KDTreeNode {
            point: pivot.clone(),
            dim: dim,
            left: left,
            right: right,
        }
    }

    pub fn find_nearest_neighbor<'a>(&'a self, point: &Point) -> (&'a Point, usize) {
        self.find_nearest_neighbor_helper(point, &self.point, (point - &self.point).norm_sq(), 1)
    }

    fn find_nearest_neighbor_helper<'a>(&'a self,
                                        point: &Point,
                                        best: &'a Point,
                                        best_dist_sq: f32,
                                        n_visited: usize)
                                        -> (&'a Point, usize) {
        let mut my_best = best;
        let mut my_best_dist_sq = best_dist_sq;
        let mut my_n_visited = n_visited;

        // We should always examine the near side
        if self.point.coords[self.dim] < point.coords[self.dim] && self.right.is_some() {
            let (a, b) = self.right
                .as_ref()
                .unwrap()
                .find_nearest_neighbor_helper(point, my_best, my_best_dist_sq, my_n_visited);
            my_best = a;
            my_n_visited = b;
        } else if self.left.is_some() {
            let (a, b) = self.left
                .as_ref()
                .unwrap()
                .find_nearest_neighbor_helper(point, my_best, my_best_dist_sq, my_n_visited);
            my_best = a;
            my_n_visited = b;
        }

        // distance along this node's axis
        let axis_dist_sq = (self.point.coords[self.dim] - point.coords[self.dim]).powi(2);
        if axis_dist_sq <= my_best_dist_sq {
            // self can only be nearer than best if axis_dist_sq is less than
            // best_dist_sq because axis_dist_sq is a lower bound for
            // self_dist_sq
            let self_dist_sq = (point - &self.point).norm_sq();
            if self_dist_sq < my_best_dist_sq {
                my_best = &self.point;
                my_best_dist_sq = self_dist_sq;
            }

            // bookkeeping
            my_n_visited += 1;

            // same reasoning applies for the far side of the split
            if self.point.coords[self.dim] < point.coords[self.dim] && self.left.is_some() {
                let (a, b) = self.left
                    .as_ref()
                    .unwrap()
                    .find_nearest_neighbor_helper(point, my_best, my_best_dist_sq, my_n_visited);
                my_best = a;
                my_n_visited = b;
            } else if self.right.is_some() {
                let (a, b) = self.right
                    .as_ref()
                    .unwrap()
                    .find_nearest_neighbor_helper(point, my_best, my_best_dist_sq, my_n_visited);
                my_best = a;
                my_n_visited = b;
            }
        }

        (my_best, my_n_visited)
    }
}

pub fn main() {
    // wordpress
    let mut wp_points: Vec<Point> =
        [[2f32, 3f32], [5f32, 4f32], [9f32, 6f32], [4f32, 7f32], [8f32, 1f32], [7f32, 2f32]]
            .iter()
            .map(|x| Point { coords: x.to_vec() })
            .collect();
    let wp_tree = KDTreeNode::new(&mut wp_points[..], 0);

    let wp_target = Point { coords: vec![9f32, 2f32] };
    let (point, n_visited) = wp_tree.find_nearest_neighbor(&wp_target);
    println!("Wikipedia example data:");
    println!("Point: [9, 2]");
    println!("Nearest neighbor: {:?}", point);
    println!("Distance: {}", (point - &wp_target).norm_sq().sqrt());
    println!("Nodes visited: {}", n_visited);

    // randomly generated 3D
    let n_random = 1000;
    let make_random_point = || {
        Point {
            coords: (0..3)
                .map(|_| (rand::thread_rng().gen::<f32>() - 0.5f32) * 1000f32)
                .collect(),
        }
    };
    let mut random_points: Vec<Point> = (0..n_random)
        .map(|_| make_random_point())
        .collect();

    let start_cons_time = get_time();
    let random_tree = KDTreeNode::new(&mut random_points[..], 0);
    let end_cons_time = get_time();
    println!("1,000 3d points (Construction time: {}ms)",
             ((end_cons_time.sec - start_cons_time.sec) * 1000) as f32 +
             ((end_cons_time.nsec - start_cons_time.nsec) as f32) / 1000000f32);

    let random_target = make_random_point();

    let (point, n_visited) = random_tree.find_nearest_neighbor(&random_target);
    println!("Point: {:?}", random_target);
    println!("Nearest neighbor: {:?}", point);
    println!("Distance: {}", (point - &random_target).norm_sq().sqrt());
    println!("Nodes visited: {}", n_visited);

    // benchmark search time
    let n_searches = 1000;
    let random_targets: Vec<Point> = (0..n_searches)
        .map(|_| make_random_point())
        .collect();

    let start_search_time = get_time();
    let mut total_n_visited = 0;
    for target in &random_targets {
        let (_, n_visited) = random_tree.find_nearest_neighbor(target);
        total_n_visited += n_visited;
    }
    let end_search_time = get_time();
    println!("Visited an average of {} nodes on {} searches in {} ms",
             total_n_visited as f32 / n_searches as f32,
             n_searches,
             ((end_search_time.sec - start_search_time.sec) * 1000) as f32 +
             ((end_search_time.nsec - start_search_time.nsec) as f32) / 1000000f32);
}

fn quickselect_by<T>(arr: &mut [T], position: usize, cmp: &Fn(&T, &T) -> Ordering) -> T
    where T: Clone
{
    let mut pivot_index = rand::thread_rng().gen_range(0, arr.len());
    // Need to wrap in another closure or we get ownership complaints.
    // Tried using an unboxed closure to get around this but couldn't get it to work.
    pivot_index = partition_by(arr, pivot_index, &|a: &T, b: &T| cmp(a, b));
    let array_len = arr.len();
    if position == pivot_index {
        arr[position].clone()
    } else if position < pivot_index {
        quickselect_by(&mut arr[0..pivot_index], position, cmp)
    } else {
        quickselect_by(&mut arr[pivot_index + 1..array_len],
                       position - pivot_index - 1,
                       cmp)
    }
}

#[cfg_attr(feature="clippy", allow(needless_range_loop))]
fn partition_by<T>(arr: &mut [T], pivot_index: usize, cmp: &Fn(&T, &T) -> Ordering) -> usize {
    let array_len = arr.len();
    arr.swap(pivot_index, array_len - 1);
    let mut store_index = 0;
    for i in 0..array_len - 1 {
        if (*cmp)(&arr[i], &arr[array_len - 1]) == Less {
            arr.swap(i, store_index);
            store_index += 1;
        }
    }
    arr.swap(array_len - 1, store_index);
    store_index
}

#[cfg(test)]
mod tests {
    use super::{Point, KDTreeNode};

    #[test]
    fn wp() {
        let mut wp_points: Vec<Point> =
            [[2f32, 3f32], [5f32, 4f32], [9f32, 6f32], [4f32, 7f32], [8f32, 1f32], [7f32, 2f32]]
                .iter()
                .map(|x| Point { coords: x.to_vec() })
                .collect();
        let wp_tree = KDTreeNode::new(&mut wp_points[..], 0);

        let wp_target = Point { coords: vec![9f32, 2f32] };
        let (point, _) = wp_tree.find_nearest_neighbor(&wp_target);
        assert_eq!(*point, Point { coords: vec![8f32, 1f32] });
    }
}
