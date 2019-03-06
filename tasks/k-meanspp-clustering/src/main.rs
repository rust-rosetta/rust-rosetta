#[macro_use]
extern crate structopt;
extern crate csv;
extern crate gnuplot;
extern crate nalgebra;
extern crate rand;

use std::f64::consts::PI;
use std::fs::File;
use std::path::PathBuf;

use gnuplot::{Axes2D, AxesCommon, Color, Figure, Fix, PointSize, PointSymbol};
use nalgebra::DVector;
use rand::distributions::Uniform;
use rand::{Rng, SeedableRng, StdRng};
use structopt::StructOpt;

type Point = DVector<f64>;

struct Cluster<'a> {
    members: Vec<&'a Point>,
    center: Point,
}

struct Stats {
    centroids: Vec<Point>,
    mean_d_from_centroid: DVector<f64>,
}

/// `DVector` doesn't implement `BaseFloat`, so a custom distance function is required.
fn sqdist(p1: &Point, p2: &Point) -> f64 {
    (p1.clone() - p2.clone()).iter().map(|x| x * x).sum()
}

/// Returns (distance^2, index) tuple of winning point.
fn nearest(p: &Point, candidates: &[Point]) -> (f64, usize) {
    let (dsquared, the_index) =
        candidates
            .iter()
            .enumerate()
            .fold((sqdist(p, &candidates[0]), 0), |(d, index), next| {
                let dprime = sqdist(p, &candidates[next.0]);
                if dprime < d {
                    (dprime, next.0)
                } else {
                    (d, index)
                }
            });
    (dsquared, the_index)
}

/// Computes starting centroids and makes initial assignments.
fn kpp(points: &[Point], k: usize, rng: &mut impl Rng) -> Stats {
    let mut centroids: Vec<Point> = Vec::new();
    // Random point for first centroid guess:
    centroids.push(rng.choose(&points).unwrap().clone());
    let mut dists: Vec<f64> = vec![0f64; points.len()];

    for _ in 1..k {
        let mut sum = 0.0;
        for (j, p) in points.iter().enumerate() {
            let (dsquared, _) = nearest(p, &centroids);
            dists[j] = dsquared;
            sum += dsquared;
        }

        // This part chooses the next cluster center with a probability proportional to d^2
        sum *= rng.gen::<f64>();
        for (j, d) in dists.iter().enumerate() {
            sum -= *d;
            if sum <= 0f64 {
                centroids.push(points[j].clone());
                break;
            }
        }
    }

    let clusters = assign_clusters(points, &centroids);
    compute_stats(&clusters)
}

fn assign_clusters<'a>(points: &'a [Point], centroids: &[Point]) -> Vec<Cluster<'a>> {
    let mut clusters: Vec<Cluster> = Vec::new();

    for _ in 0..centroids.len() {
        clusters.push(Cluster {
            members: Vec::new(),
            center: DVector::zeros(points[0].len()),
        });
    }

    for p in points.iter() {
        let (_, nearest_index) = nearest(p, centroids);
        clusters[nearest_index].center = clusters[nearest_index].center.clone() + p.clone();
        clusters[nearest_index].members.push(p);
    }

    for cluster in &mut clusters {
        cluster.center = cluster.center.clone() / cluster.members.len() as f64;
    }

    clusters
}

/// Computes centroids and mean-distance-from-centroid for each cluster.
fn compute_stats(clusters: &[Cluster]) -> Stats {
    let mut centroids = Vec::new();
    let mut means_vec = Vec::new();

    for c in clusters.iter() {
        let pts = &c.members;
        let seed: DVector<f64> = DVector::zeros(pts[0].len());
        let centroid = pts.iter().fold(seed, |a, &b| a + b.clone()) / pts.len() as f64;
        means_vec.push(
            pts.iter()
                .fold(0f64, |acc, pt| acc + sqdist(pt, &centroid).sqrt())
                / pts.len() as f64,
        );
        centroids.push(centroid);
    }

    Stats {
        centroids: centroids,
        mean_d_from_centroid: DVector::from_row_slice(means_vec.len(), means_vec.as_slice()),
    }
}

fn lloyd<'a>(
    points: &'a [Point],
    k: usize,
    stoppage_delta: f64,
    max_iter: u32,
    rng: &mut impl Rng,
) -> (Vec<Cluster<'a>>, Stats) {
    let mut clusters = Vec::new();
    // Choose starting centroids and make initial assignments
    let mut stats = kpp(points, k, rng);

    for i in 1..max_iter {
        let last_means: DVector<f64> = stats.mean_d_from_centroid.clone();
        clusters = assign_clusters(points, &stats.centroids);
        stats = compute_stats(&clusters);
        let err = sqdist(&stats.mean_d_from_centroid, &last_means).sqrt();
        if err < stoppage_delta {
            println!("Stoppage condition reached on iteration {}", i);
            return (clusters, stats);
        }
        // Console output
        print!("Iter {}: ", i);
        for (cen, mu) in stats
            .centroids
            .iter()
            .zip(stats.mean_d_from_centroid.iter())
        {
            print_dvec(cen);
            print!(" {:1.2} | ", mu);
        }
        print!("{:1.5}\n", err);
    }

    println!("Stoppage condition not reached by iteration {}", max_iter);
    (clusters, stats)
}

/// Uniform sampling on the unit disk.
fn generate_points(n: u32, rng: &mut impl Rng) -> Vec<Point> {
    // `Uniform` rather than `gen_range`'s `Uniform::sample_single` for speed
    let range = Uniform::new(0.0, 2.0 * PI);

    (0..n)
        .map(|_| {
            let root_r = rng.gen::<f64>();
            let theta = rng.sample(range);
            DVector::<f64>::from_row_slice(2, &[root_r * theta.cos(), root_r * theta.sin()])
        })
        .collect()
}

// Plot clusters (2d only). Closure idiom allows us to borrow and mutate the Axes2D.
fn viz(clusters: &[Cluster], stats: &Stats, k: usize, n: u32, e: f64) {
    let mut fg = Figure::new();
    {
        let prep = |fg: &mut Figure| {
            let axes: &mut Axes2D = fg.axes2d();
            let title: String = format!("k = {}, n = {}, e = {:4}", k, n, e);
            let centroids_x = stats.centroids.iter().map(|c| c[0]);
            let centroids_y = stats.centroids.iter().map(|c| c[1]);
            for cluster in clusters.iter() {
                axes.points(
                    cluster.members.iter().map(|p| p[0]),
                    cluster.members.iter().map(|p| p[1]),
                    &[PointSymbol('O'), PointSize(0.25)],
                );
            }
            axes.set_aspect_ratio(Fix(1.0))
                .points(
                    centroids_x,
                    centroids_y,
                    &[PointSymbol('o'), PointSize(1.5), Color("black")],
                )
                .set_title(&title[..], &[]);
        };
        prep(&mut fg);
    }
    fg.show();
}

fn print_dvec(v: &DVector<f64>) {
    print!("(");
    for elem in v.iter().take(v.len() - 1) {
        print!("{:+1.2}, ", elem)
    }
    print!("{:+1.2})", v.iter().last().unwrap());
}

fn unseeded_stdrng() -> StdRng {
    let mut seed = <StdRng as SeedableRng>::Seed::default();
    for (i, x) in seed.iter_mut().enumerate() {
        *x = i as u8;
    }
    StdRng::from_seed(seed)
}

#[derive(Debug, StructOpt)]
struct Opt {
    /// Number of clusters to assign
    #[structopt(short = "k", default_value = "7")]
    clusters: usize,

    /// Operate on this many points on the unit disk
    #[structopt(short = "n", default_value = "30000")]
    points: u32,

    /// Min delta in norm of successive cluster centroids to continue
    #[structopt(short = "e", default_value = "1e-3")]
    epsilon: f64,

    /// Read points from file (overrides -n)
    #[structopt(short = "f", parse(from_os_str))]
    csv: Option<PathBuf>,
}

fn main() {
    let mut opt = Opt::from_args();
    const MAX_ITERATIONS: u32 = 100u32;

    let mut rng = unseeded_stdrng();

    let points = if let Some(filename) = opt.csv {
        let mut points = Vec::new();
        let mut rdr = csv::Reader::from_reader(File::open(&filename).unwrap());
        for row in rdr.deserialize() {
            let floats: Vec<f64> = row.unwrap();
            points.push(DVector::<f64>::from_row_slice(
                floats.len(),
                floats.as_slice(),
            ));
        }
        assert!(points.iter().all(|v| v.len() == points[0].len()));
        opt.points = points.len() as u32;
        println!("Read {} points from {}", points.len(), filename.display());
        points
    } else {
        // Proceed with random 2d data
        generate_points(opt.points, &mut rng)
    };

    assert!(points.len() >= opt.clusters);
    let (clusters, stats) = lloyd(&points, opt.clusters, opt.epsilon, MAX_ITERATIONS, &mut rng);

    println!(
        " k       centroid{}mean dist    pop",
        std::iter::repeat(" ")
            .take((points[0].len() - 2) * 7 + 7)
            .collect::<String>()
    );
    println!(
        "===  {}  ===========  =====",
        std::iter::repeat("=")
            .take(points[0].len() * 7 + 2)
            .collect::<String>()
    );
    for (i, cluster) in clusters.iter().enumerate() {
        print!(" {:>1}    ", i);
        print_dvec(&stats.centroids[i]);
        print!(
            "      {:1.2}       {:>4}\n",
            stats.mean_d_from_centroid[i],
            cluster.members.len()
        );
    }

    if points[0].len() == 2 {
        viz(&clusters, &stats, opt.clusters, opt.points, opt.epsilon)
    }
}

#[cfg(test)]
mod tests {
    use super::{generate_points, lloyd, unseeded_stdrng};

    #[test]
    fn test_lloyd2d() {
        let mut rng = unseeded_stdrng();
        let points = generate_points(1000, &mut rng);

        let (clusters, stats) = lloyd(&points, 4, 0.001, 100, &mut rng);

        assert!(clusters.len() == 4);
        for i in 0..clusters.len() {
            assert!(clusters[i].members.len() > 0);
        }
        assert!(stats.mean_d_from_centroid.iter().all(|d| *d > 0f64));
        assert!(stats.centroids.iter().any(|p| p[0] >= 0f64 && p[1] >= 0f64));
        assert!(stats.centroids.iter().any(|p| p[0] >= 0f64 && p[1] < 0f64));
        assert!(stats.centroids.iter().any(|p| p[0] < 0f64 && p[1] < 0f64));
        assert!(stats.centroids.iter().any(|p| p[0] < 0f64 && p[1] >= 0f64));
    }
}
