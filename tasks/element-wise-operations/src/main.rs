struct Matrix {
    elements: Vec<f32>,
    pub height: u32,
    pub width: u32,
}

impl Matrix {
    fn new(elements: Vec<f32>, height: u32, width: u32) -> Matrix {
        // Should check for dimensions but omitting to be succient
        Matrix {
            elements: elements,
            height: height,
            width: width,
        }
    }

    fn get(&self, row: u32, col: u32) -> f32 {
        let row = row as usize;
        let col = col as usize;
        self.elements[col + row * (self.width as usize)]
    }

    fn set(&mut self, row: u32, col: u32, value: f32) {
        let row = row as usize;
        let col = col as usize;
        self.elements[col + row * (self.width as usize)] = value;
    }

    fn print(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                print!("{:3.0}", self.get(row, col));
            }
            println!("");
        }
        println!("");
    }
}

// Matrix addition will perform element-wise addition
fn matrix_addition(first: &Matrix, second: &Matrix) -> Result<Matrix, String> {
    if first.width == second.width && first.height == second.height {
        let mut result = Matrix::new(vec![0.0f32; (first.height * first.width) as usize],
                                     first.height,
                                     first.width);
        for row in 0..first.height {
            for col in 0..first.width {
                let first_value = first.get(row, col);
                let second_value = second.get(row, col);
                result.set(row, col, first_value + second_value);
            }
        }
        Ok(result)
    } else {
        Err("Dimensions don't match".to_owned())
    }
}

fn scalar_multiplication(scalar: f32, matrix: &Matrix) -> Matrix {
    let mut result = Matrix::new(vec![0.0f32; (matrix.height * matrix.width) as usize],
                                 matrix.height,
                                 matrix.width);
    for row in 0..matrix.height {
        for col in 0..matrix.width {
            let value = matrix.get(row, col);
            result.set(row, col, scalar * value);
        }
    }
    result
}

// Subtract second from first
fn matrix_subtraction(first: &Matrix, second: &Matrix) -> Result<Matrix, String> {
    if first.width == second.width && first.height == second.height {
        let negative_matrix = scalar_multiplication(-1.0, second);
        let result = matrix_addition(first, &negative_matrix).unwrap();
        Ok(result)
    } else {
        Err("Dimensions don't match".to_owned())
    }
}

// First must be a l x m matrix and second a m x n matrix for this to work.
fn matrix_multiplication(first: &Matrix, second: &Matrix) -> Result<Matrix, String> {
    if first.width == second.height {
        let mut result = Matrix::new(vec![0.0f32; (first.height * second.width) as usize],
                                     first.height,
                                     second.width);
        for row in 0..result.height {
            for col in 0..result.width {
                let mut value = 0.0;
                for it in 0..first.width {
                    value += first.get(row, it) * second.get(it, col);
                }
                result.set(row, col, value);
            }
        }
        Ok(result)
    } else {
        Err("Dimensions don't match. Width of first must equal height of second".to_owned())
    }
}


fn main() {
    let height = 2;
    let width = 3;
    // Matrix will look like:
    // | 1.0  2.0  3.0  |
    // | 4.0  5.0  6.0 |
    let matrix1 = Matrix::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], height, width);

    // Matrix will look like:
    // | 6.0  5.0  4.0  |
    // | 3.0  2.0  1.0 |
    let matrix2 = Matrix::new(vec![6.0, 5.0, 4.0, 3.0, 2.0, 1.0], height, width);

    // | 7.0  7.0  7.0  |
    // | 7.0  7.0  7.0 |
    matrix_addition(&matrix1, &matrix2).unwrap().print();
    // | 2.0   4.0   6.0  |
    // | 8.0  10.0  12.0 |
    scalar_multiplication(2.0, &matrix1).print();
    // | -5.0  -3.0  -1.0  |
    // |  1.0   3.0   5.0 |
    matrix_subtraction(&matrix1, &matrix2).unwrap().print();

    // | 1.0 |
    // | 1.0 |
    // | 1.0 |
    let matrix3 = Matrix::new(vec![1.0, 1.0, 1.0], width, 1);
    // |  6 |
    // | 15 |
    matrix_multiplication(&matrix1, &matrix3).unwrap().print();
}

#[cfg(test)]
mod tests {
    use super::{Matrix, matrix_addition, scalar_multiplication, matrix_subtraction,
                matrix_multiplication};

    const HEIGHT: u32 = 2;
    const WIDTH: u32 = 3;
    #[test]
    fn matrix_addition_test() {
        let matrix1: Matrix = Matrix::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], HEIGHT, WIDTH);
        let matrix2: Matrix = Matrix::new(vec![6.0, 5.0, 4.0, 3.0, 2.0, 1.0], HEIGHT, WIDTH);
        let result = matrix_addition(&matrix1, &matrix2).unwrap();
        for col in 0..result.width {
            for row in 0..result.height {
                assert_eq!(result.get(row, col), 7.0);
            }
        }

    }

    #[test]
    fn scalar_multiplication_test() {
        let matrix1: Matrix = Matrix::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], HEIGHT, WIDTH);
        let result = scalar_multiplication(2.0, &matrix1);
        assert_eq!(result.get(0, 0), 2.0);
        assert_eq!(result.get(0, 1), 4.0);
        assert_eq!(result.get(0, 2), 6.0);
        assert_eq!(result.get(1, 0), 8.0);
        assert_eq!(result.get(1, 1), 10.0);
        assert_eq!(result.get(1, 2), 12.0);
    }

    #[test]
    fn matrix_subtraction_test() {
        let matrix1: Matrix = Matrix::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], HEIGHT, WIDTH);
        let matrix2: Matrix = Matrix::new(vec![6.0, 5.0, 4.0, 3.0, 2.0, 1.0], HEIGHT, WIDTH);
        let result = matrix_subtraction(&matrix1, &matrix2).unwrap();
        assert_eq!(result.get(0, 0), -5.0);
        assert_eq!(result.get(0, 1), -3.0);
        assert_eq!(result.get(0, 2), -1.0);
        assert_eq!(result.get(1, 0), 1.0);
        assert_eq!(result.get(1, 1), 3.0);
        assert_eq!(result.get(1, 2), 5.0);
    }

    #[test]
    fn matrix_multiplication_test() {
        let matrix1: Matrix = Matrix::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], HEIGHT, WIDTH);
        let matrix2 = Matrix::new(vec![1.0, 1.0, 1.0], WIDTH, 1);
        let result = matrix_multiplication(&matrix1, &matrix2).unwrap();
        assert_eq!(result.get(0, 0), 6.0);
        assert_eq!(result.get(1, 0), 15.0);
    }
}
