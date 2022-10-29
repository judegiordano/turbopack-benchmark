use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate turbopack_benchmark;
use turbopack_benchmark::builder;

const DIR: &str = "twoyay-web";

fn turbo_vs_next(c: &mut Criterion) {
    let mut group = c.benchmark_group("turbo-vs-next-build");
    group.significance_level(0.1).sample_size(10);
    group.bench_function("npx next build", |b| {
        b.iter(|| builder(black_box(DIR), black_box(false)))
    });
    group.bench_function("npx turbo build", |b| {
        b.iter(|| builder(black_box(DIR), black_box(true)))
    });
    group.finish();
}

criterion_group!(benches, turbo_vs_next);
criterion_main!(benches);
