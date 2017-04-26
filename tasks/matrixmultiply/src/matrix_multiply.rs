struct Matrix {
    dat: [[f32; 3]; 3]
}
 
impl Matrix {
    pub fn mult_m(a: Matrix, b: Matrix) -> Matrix
    {
        let mut out = Matrix {
            dat: [[0., 0., 0.],
                  [0., 0., 0.],
                  [0., 0., 0.]
                  ]
        };
 
        for i in 0..3{
            for j in 0..3 {
                for k in 0..3 {
                    out.dat[i][j] += a.dat[i][k] * b.dat[k][j];
                }
            }
        }
 
        out
    }
 
    pub fn print(self)
    {
        for i in 0..3 {
            for j in 0..3 {
                print!("{} ", self.dat[i][j]);
            }
            print!("\n");
        }
    }
}
 
fn main()
{
    let  a = Matrix {
        dat: [[1., 2., 3.],
              [4., 5., 6.],
              [7., 8., 9.]
              ]
    };
 
    let  b = Matrix {
        dat: [[1., 0., 0.],
              [0., 1., 0.],
              [0., 0., 1.]]
    };
 
 
 
        let c = Matrix::mult_m(a, b);
 
 
    c.print();
}
