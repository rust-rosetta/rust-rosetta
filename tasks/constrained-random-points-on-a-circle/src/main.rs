#![feature(inclusive_range_syntax)]

extern crate rand;

use rand::Rng;

const POINTS_N: usize = 100;

fn generate_point<R: Rng>(rng: &mut R) -> (i32, i32) {
    loop {
        let x = rng.gen_range(-15, 16); // exclusive
        let y = rng.gen_range(-15, 16);

        let r2 = x * x + y * y;
        if r2 >= 100 && r2 <= 225 {
            return (x, y);
        }
    }
}

fn filtering_method<R: Rng>(rng: &mut R) {
    let mut rows = [[" "; 62]; 31];

    // Generate points
    for _ in 0..POINTS_N {
        let (x, y) = generate_point(rng);
        rows[(y + 15) as usize][(x + 15) as usize * 2] = "*";
    }

    // draw the points
    for row in &rows {
        println!("{}", row.concat());
    }
}

fn precalculating_method<R: Rng>(rng: &mut R) {
    // Generate all possible points
    let mut possible_points = Vec::with_capacity(404);
    for y in -15..=15 {
        for x in -15..=15 {
            let r2 = x * x + y * y;
            if r2 >= 100 && r2 <= 225 {
                possible_points.push((x, y));
            }
        }
    }

    // A truncated Fisher-Yates shuffle
    let len = possible_points.len();
    for i in (len - POINTS_N..len).rev() {
        let j = rng.gen_range(0, i + 1);
        possible_points.swap(i, j);
    }

    // turn the selected points into "pixels"
    let mut rows = [[" "; 62]; 31];
    for &(x, y) in &possible_points[len - POINTS_N..] {
        rows[(y + 15) as usize][(x + 15) as usize * 2] = "*";
    }

    // draw the "pixels"
    for row in &rows {
        println!("{}", row.concat());
    }
}

fn main() {
    let mut rng = rand::weak_rng();

    filtering_method(&mut rng);

    precalculating_method(&mut rng);
}
