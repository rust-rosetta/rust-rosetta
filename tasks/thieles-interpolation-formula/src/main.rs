const N: usize = 32;
const STEP: f64 = 0.05;

fn main() {
    let x: Vec<f64> = (0..N).map(|i| i as f64 * STEP).collect();
    let sin = x.iter().map(|x| x.sin()).collect::<Vec<_>>();
    let cos = x.iter().map(|x| x.cos()).collect::<Vec<_>>();
    let tan = x.iter().map(|x| x.tan()).collect::<Vec<_>>();

    println!(
        "{}\n{}\n{}",
        6. * thiele(&sin, &x, 0.5),
        3. * thiele(&cos, &x, 0.5),
        4. * thiele(&tan, &x, 1.)
    );
}

fn thiele(x: &[f64], y: &[f64], xin: f64) -> f64 {
    let mut p: Vec<Vec<f64>> = (0..N).map(|i| (i..N).map(|_| 0.0).collect()).collect();

    (0..N).for_each(|i| p[i][0] = y[i]);

    (0..N - 1).for_each(|i| p[i][1] = (x[i] - x[i + 1]) / (p[i][0] - p[i + 1][0]));

    (2..N).for_each(|i| {
        (0..N - i).for_each(|j| {
            p[j][i] = (x[j] - x[j + i]) / (p[j][i - 1] - p[j + 1][i - 1]) + p[j + 1][i - 2];
        })
    });

    let mut a = 0.;
    (2..N).rev().for_each(|i| {
        a = (xin - x[i - 1]) / (p[0][i] - p[0][i - 2] + a);
    });
    y[0] + (xin - x[0]) / (p[0][1] + a)
}
