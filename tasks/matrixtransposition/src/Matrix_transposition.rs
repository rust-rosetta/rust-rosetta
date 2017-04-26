 
struct Matrix {
    dat: [[i32; 3]; 3]
}
 
 
 
impl Matrix {
    pub fn transpose_m(a: Matrix) -> Matrix
    {
        let mut out = Matrix {
            dat: [[0, 0, 0],
                  [0, 0, 0],
                  [0, 0, 0]
                  ]
        };
 
        for i in 0..3{
            for j in 0..3{
 
                    out.dat[i][j] = a.dat[j][i];
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
        dat: [[1, 2, 3],
              [4, 5, 6],
              [7, 8, 9] ]
    };
 
let c = Matrix::transpose_m(a);
    c.print();
}
