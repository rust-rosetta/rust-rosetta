use bernoulli_numbers::{bernoulli, bernoulli_naive, Context};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark(c: &mut Criterion) {
    c.bench_function("naive", |b| {
        b.iter(|| {
            let mut context = Context::new();

            for n in 0..=30 {
                bernoulli_naive(black_box(n), &mut context);
            }
        })
    });

    c.bench_function("naive", |b| {
        b.iter(|| {
            let mut context = Context::new();

            for n in 0..=30 {
                bernoulli(black_box(n), &mut context);
            }
        })
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
