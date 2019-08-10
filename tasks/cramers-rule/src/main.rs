use std::ops::{Index, IndexMut};

fn main() {
    let m = matrix(
        vec![
            2., -1., 5., 1., 3., 2., 2., -6., 1., 3., 3., -1., 5., -2., -3., 3.,
        ],
        4,
    );
    let mm = m.solve(&vec![-3., -32., -47., 49.]);
    println!("{:?}", mm);
}

#[derive(Clone)]
struct Matrix {
    elts: Vec<f64>,
    dim: usize,
}

impl Matrix {
    // Compute determinant using cofactor method
    // Using Gaussian elimination would have been more efficient, but it also solves the linear
    // system, so…
    fn det(&self) -> f64 {
        match self.dim {
            0 => 0.,
            1 => self[0][0],
            2 => self[0][0] * self[1][1] - self[0][1] * self[1][0],
            d => {
                let mut acc = 0.;
                let mut signature = 1.;
                for k in 0..d {
                    acc += signature * self[0][k] * self.comatrix(0, k).det();
                    signature *= -1.
                }
                acc
            }
        }
    }

    // Solve linear systems using Cramer's method
    fn solve(&self, target: &Vec<f64>) -> Vec<f64> {
        let mut solution: Vec<f64> = vec![0.; self.dim];
        let denominator = self.det();
        for j in 0..self.dim {
            let mut mm = self.clone();
            for i in 0..self.dim {
                mm[i][j] = target[i]
            }
            solution[j] = mm.det() / denominator
        }
        solution
    }

    // Compute the cofactor matrix for determinant computations
    fn comatrix(&self, k: usize, l: usize) -> Matrix {
        let mut v: Vec<f64> = vec![];
        for i in 0..self.dim {
            for j in 0..self.dim {
                if i != k && j != l {
                    v.push(self[i][j])
                }
            }
        }
        matrix(v, self.dim - 1)
    }
}

fn matrix(elts: Vec<f64>, dim: usize) -> Matrix {
    assert_eq!(elts.len(), dim * dim);
    Matrix { elts, dim }
}

impl Index<usize> for Matrix {
    type Output = [f64];

    fn index(&self, i: usize) -> &Self::Output {
        let m = self.dim;
        &self.elts[m * i..m * (i + 1)]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        let m = self.dim;
        &mut self.elts[m * i..m * (i + 1)]
    }
}

#[test]
fn test_create_and_access() {
    let m = matrix(vec![1., 2., 3., 4.], 2);
    assert_eq!(m[0][0], 1.);
    assert_eq!(m[0][1], 2.);
    assert_eq!(m[1][0], 3.);
    assert_eq!(m[1][1], 4.);
}

#[test]
fn test_determinant() {
    let dim = 5;
    let mut m = matrix(vec![0.; dim * dim], dim);
    (0..dim).for_each(|i| (0..=i).for_each(|j| m[i][j] = 1.));
    assert_eq!(m.det(), 1.);
    let m = matrix(
        vec![
            1., -5., 2., 1., -5., -7., 0., 3., 0., 3., 4., 1., 41., 0., -1., 0.,
        ],
        4,
    );
    assert_eq!(m.det(), -2680.);
}

#[test]
fn test_solve() {
    let m = matrix(
        vec![
            1., -5., 2., 1., -5., -7., 0., 3., 0., 3., 4., 1., 41., 0., -1., 0.,
        ],
        4,
    );
    let b = vec![153., 219., -94., -41.];
    assert_eq!(m.solve(&b), vec![-1., -31., 0., -1.])
}
