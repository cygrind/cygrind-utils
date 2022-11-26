use criterion::{criterion_group, criterion_main, Criterion};
use cygrind_utils::{draw2d::draw::Draw2d, util};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("draw2d", |b| b.iter(|| Draw2d::draw(util::random_pattern())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
