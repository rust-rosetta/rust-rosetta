//! Translation of C.

use std::f64;
use std::fmt;

#[derive(Clone,Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn distance(p1: Point, p2: Point) -> f64 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.4}, {:.4})", self.x, self.y)
    }
}

fn describe_circle(p1: Point, p2: Point, r: f64) {
    let sep = distance(p1, p2);

    if (sep - 0.).abs() < f64::EPSILON {
        if r == 0. {
            println!("No circles can be drawn through {}", p1);
        } else {
            println!("Infinitely many circles can be drawn through {}", p1);
        }
    } else if (sep - 2.0 * r).abs() < f64::EPSILON {
        println!("Given points are opposite ends of a diameter of the circle with center \
                  ({:.4},{:.4}) and r {:.4}",
                 (p1.x + p2.x) / 2.0,
                 (p1.y + p2.y) / 2.0,
                 r);
    } else if (sep - 2.0 * r).abs() < f64::EPSILON {
        println!("Given points are farther away from each other than a diameter of a circle with \
                  r {:.4}",
                 r);
    } else {
        let mirror_dist = (r.powi(2) - (sep / 2.0).powi(2)).sqrt();

        println!("Two circles are possible.");
        println!("Circle C1 with center ({:.4}, {:.4}), r {:.4} and Circle C2 with center \
                  ({:.4}, {:.4}), r {:.4}",
                 ((p1.x + p2.x) / 2.0) + mirror_dist * (p1.y - p2.y) / sep,
                 (p1.y + p2.y) / 2.0 + mirror_dist * (p2.x - p1.x) / sep,
                 r,
                 (p1.x + p2.x) / 2.0 - mirror_dist * (p1.y - p2.y) / sep,
                 (p1.y + p2.y) / 2.0 - mirror_dist * (p2.x - p1.x) / sep,
                 r);
    }
}

fn main() {
    let points = vec![(Point {
                          x: 0.1234,
                          y: 0.9876,
                      },
                       Point {
                          x: 0.8765,
                          y: 0.2345,
                      }),
                      (Point {
                          x: 0.0000,
                          y: 2.0000,
                      },
                       Point {
                          x: 0.0000,
                          y: 0.0000,
                      }),
                      (Point {
                          x: 0.1234,
                          y: 0.9876,
                      },
                       Point {
                          x: 0.1234,
                          y: 0.9876,
                      }),
                      (Point {
                          x: 0.1234,
                          y: 0.9876,
                      },
                       Point {
                          x: 0.8765,
                          y: 0.2345,
                      }),
                      (Point {
                          x: 0.1234,
                          y: 0.9876,
                      },
                       Point {
                          x: 0.1234,
                          y: 0.9876,
                      })];
    let radii: Vec<f64> = vec![2.0, 1.0, 2.0, 0.5, 0.0];

    for (p, r) in points.into_iter().zip(radii.into_iter()) {
        println!("\nPoints: ({}, {}), Radius: {:.4}", p.0, p.1, r);
        describe_circle(p.0, p.1, r);
    }
}
