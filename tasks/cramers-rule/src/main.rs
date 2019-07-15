fn main() {
    let m = matrix_from_vec(vec![2, -1, 5, 1, 3, 2, 2, -6, 1, 3, 3, -1, 5, -2, -3, 3], 4);
    let mm = m.solve(&vec![-3, -32, -47, 49]);
    println!("{:?}", mm);
}

#[derive(Clone)]
struct Matrix {
    elts: Vec<Vec<i64>>,
    dim: usize,
}

impl Matrix {
    // Compute determinant with the cofactor method
    // Using Gaussian elimination would have been more efficient, but it also solves the linear
    // system, so…
    fn det(&self) -> f64 {
        match self.dim {
            0 => 0.,
            1 => self.elts[0][0] as f64,
            2 => (self.elts[0][0] * self.elts[1][1] - self.elts[0][1] * self.elts[1][0]) as f64,
            d => {
                let mut acc = 0.;
                let mut coeff = 1.;
                for k in 0..d {
                    acc += coeff * self.elts[0][k] as f64 * self.comatrix(0, k).det();
                    coeff *= -1.
                }
                acc
            }
        }
    }

    // Solve linear systems using Cramer's method
    fn solve(&self, target: &Vec<i64>) -> Vec<f64> {
        let mut solution: Vec<f64> = vec![0.; self.dim];
        let denominator = self.det();
        for j in 0..self.dim {
            let mut mm = self.clone();
            for i in 0..self.dim {
                mm.elts[i][j] = target[i]
            }
            solution[j] = mm.det() / denominator
        }
        solution
    }

    // Compute a cofactor matrix for determinant computation
    fn comatrix(&self, k: usize, l: usize) -> Matrix {
        let mut v: Vec<i64> = vec![];
        for i in 0..self.dim {
            for j in 0..self.dim {
                if i != k && j != l {
                    v.push(self.elts[i][j])
                }
            }
        }
        matrix_from_vec(v, self.dim - 1)
    }
}

fn matrix_from_vec(v: Vec<i64>, dim: usize) -> Matrix {
    assert_eq!(v.len(), dim * dim);
    let mut elts = vec![vec!(0, dim); dim];
    for i in 0..dim {
        for j in 0..dim {
            elts[i][j] = v[i * dim + j]
        }
    }
    Matrix { elts, dim }
}
