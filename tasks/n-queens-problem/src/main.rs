use n_queens_problem::{n_queens, semi_parallel_n_queens};

fn main() {
    for num in 0i32..16 {
        println!("Sequential: {}: {}", num, n_queens(num));
    }
    for num in 0i32..16 {
        println!("Parallel: {}: {}", num, semi_parallel_n_queens(num));
    }
}
