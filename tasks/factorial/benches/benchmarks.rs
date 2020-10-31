use factorial::{factorial_iterative, factorial_loop, factorial_recursive};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark(c: &mut Criterion) {
    c.bench_function("iterative", |b| {
        b.iter(|| factorial_iterative(black_box(20)))
    });
    c.bench_function("recursive", |b| {
        b.iter(|| factorial_recursive(black_box(20)))
    });
    c.bench_function("loop", |b| b.iter(|| factorial_loop(black_box(20))));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
