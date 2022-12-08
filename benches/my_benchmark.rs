use criterion::{black_box, criterion_group, criterion_main, Criterion};
use advent_of_code_2022::days::solve;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve 20", |b| b.iter(|| solve()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
