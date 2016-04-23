// http://rosettacode.org/wiki/Closest-pair_problem

//! We interpret complex numbers as points in the Cartesian plane, here. We also use the
//! [sweepline/plane sweep closest pairs algorithm][algorithm] instead of the divide-and-conquer
//! algorithm, since it's (arguably) easier to implement, and an efficient implementation does not
//! require use of unsafe.
//!
//! [algorithm]: http://www.cs.mcgill.ca/~cs251/ClosestPair/ClosestPairPS.html
extern crate num;

use std::cmp::{PartialOrd, Ordering};
use num::complex::Complex;
use std::collections::BTreeSet;
type Point = Complex<f32>;

/// Wrapper around `Point` (i.e. `Complex<f32>`) so that we can use a `TreeSet`
#[derive(PartialEq)]
struct YSortedPoint {
    point: Point,
}

impl PartialOrd for YSortedPoint {
    fn partial_cmp(&self, other: &YSortedPoint) -> Option<Ordering> {
        (self.point.im, self.point.re).partial_cmp(&(other.point.im, other.point.re))
    }
}

impl Ord for YSortedPoint {
    fn cmp(&self, other: &YSortedPoint) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for YSortedPoint {}

fn closest_pair(points: &mut [Point]) -> Option<(Point, Point)> {
    if points.len() < 2 {
        return None;
    }

    points.sort_by(|a, b| (a.re, a.im).partial_cmp(&(b.re, b.im)).unwrap());

    let mut closest_pair = (points[0], points[1]);
    let mut closest_distance_sqr = (points[0] - points[1]).norm_sqr();
    let mut closest_distance = closest_distance_sqr.sqrt();

    // the strip that we inspect for closest pairs as we sweep right
    let mut strip: BTreeSet<YSortedPoint> = BTreeSet::new();
    strip.insert(YSortedPoint { point: points[0] });
    strip.insert(YSortedPoint { point: points[1] });

    // index of the leftmost point on the strip (on points)
    let mut leftmost_idx = 0;

    // Start the sweep!
    for (idx, point) in points.iter().enumerate().skip(2) {
        // Remove all points farther than `closest_distance` away from `point`
        // along the x-axis
        while leftmost_idx < idx {
            let leftmost_point = &points[leftmost_idx];
            if (leftmost_point.re - point.re).powi(2) < closest_distance_sqr {
                break;
            }
            strip.remove(&YSortedPoint { point: *leftmost_point });
            leftmost_idx += 1;
        }

        // Compare to points in bounding box
        {
            let low_bound = YSortedPoint {
                point: Point {
                    re: ::std::f32::INFINITY,
                    im: point.im - closest_distance,
                },
            };
            let mut strip_iter = strip.iter().skip_while(|&p| p < &low_bound);
            loop {
                let point2 = match strip_iter.next() {
                    None => break,
                    Some(p) => p.point,
                };
                if point2.im - point.im >= closest_distance {
                    // we've reached the end of the box
                    break;
                }
                let dist_sqr = (*point - point2).norm_sqr();
                if dist_sqr < closest_distance_sqr {
                    closest_pair = (point2, *point);
                    closest_distance_sqr = dist_sqr;
                    closest_distance = dist_sqr.sqrt();
                }
            }
        }

        // Insert point into strip
        strip.insert(YSortedPoint { point: *point });
    }

    Some(closest_pair)
}

pub fn main() {
    let mut test_data = [Complex::new(0.654682, 0.925557),
                         Complex::new(0.409382, 0.619391),
                         Complex::new(0.891663, 0.888594),
                         Complex::new(0.716629, 0.996200),
                         Complex::new(0.477721, 0.946355),
                         Complex::new(0.925092, 0.818220),
                         Complex::new(0.624291, 0.142924),
                         Complex::new(0.211332, 0.221507),
                         Complex::new(0.293786, 0.691701),
                         Complex::new(0.839186, 0.728260)];
    let (p1, p2) = closest_pair(&mut test_data[..]).unwrap();
    println!("Closest pair: {} and {}", p1, p2);
    println!("Distance: {}", (p1 - p2).norm_sqr().sqrt());
}

#[cfg(test)]
mod tests {
    use super::closest_pair;
    use num::complex::Complex;

    #[test]
    fn random_floats() {
        let mut test_data = [Complex::new(0.654682, 0.925557),
                             Complex::new(0.409382, 0.619391),
                             Complex::new(0.891663, 0.888594),
                             Complex::new(0.716629, 0.996200),
                             Complex::new(0.477721, 0.946355),
                             Complex::new(0.925092, 0.818220),
                             Complex::new(0.624291, 0.142924),
                             Complex::new(0.211332, 0.221507),
                             Complex::new(0.293786, 0.691701),
                             Complex::new(0.839186, 0.728260)];
        let (p1, p2) = closest_pair(&mut test_data[..]).unwrap();
        assert!((p1.re - 0.891663).abs() < 1e-6f32);
        assert!((p1.im - 0.888594).abs() < 1e-6f32);
        assert!((p2.re - 0.925092).abs() < 1e-6f32);
        assert!((p2.im - 0.818220).abs() < 1e-6f32);
        assert!(((p1 - p2).norm_sqr() - 0.0779102f32.powi(2)).abs() < 1e-6f32);
    }
}
