use criterion::{criterion_group, criterion_main, Criterion};
use day01::solve_day;
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Day 01", |b| b.iter(|| solve_day()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
