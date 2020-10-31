use criterion::{black_box, criterion_group, criterion_main, Criterion};

use n_queens_problem::{n_queens, semi_parallel_n_queens};

fn benchmark(c: &mut Criterion) {
    c.bench_function("n_queens", |b| b.iter(|| n_queens(black_box(16))));
    c.bench_function("semi_parallel", |b| {
        b.iter(|| semi_parallel_n_queens(black_box(16)))
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = benchmark);
criterion_main!(benches);
