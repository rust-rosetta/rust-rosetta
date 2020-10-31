use criterion::{black_box, criterion_group, criterion_main, Criterion};

use s_expressions::SExp::*;
use s_expressions::{ParseContext, SExp};

const SEXP_STRING_IN: &str = r#"((data "quoted data" 123 4.5)
(data (!@# (4.5) "(more" "data)")))"#;

const SEXP_STRUCT: SExp<'static> = List(&[
    List(&[Str("data"), Str("quoted data"), F64(123.), F64(4.5)]),
    List(&[
        Str("data"),
        List(&[Str("!@#"), List(&[F64(4.5)]), Str("(more"), Str("data)")]),
    ]),
]);

fn benchmark(c: &mut Criterion) {
    c.bench_function("encoding", |b| {
        b.iter(|| {
            black_box(SEXP_STRUCT).buffer_encode().unwrap();
        })
    });

    c.bench_function("decoding", |b| {
        b.iter(|| {
            let ctx = &mut ParseContext::new(SEXP_STRING_IN);
            SExp::parse(black_box(ctx)).unwrap();
        })
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
