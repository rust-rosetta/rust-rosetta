use criterion::{black_box, criterion_group, criterion_main, Criterion};

use pangram_checker::{is_pangram_via_bitmask, is_pangram_via_hashset, is_pangram_via_sort};

static PANGRAM: &str = "The quick brown fox jumps over the lazy dog";

fn benchmark(c: &mut Criterion) {
    c.bench_function("sort", |b| {
        b.iter(|| is_pangram_via_sort(black_box(PANGRAM)))
    });
    c.bench_function("bitmask", |b| {
        b.iter(|| is_pangram_via_bitmask(black_box(PANGRAM)))
    });
    c.bench_function("hashset", |b| {
        b.iter(|| is_pangram_via_hashset(black_box(PANGRAM)))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
