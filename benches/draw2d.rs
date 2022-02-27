use criterion::{criterion_group, criterion_main, Criterion};
use cygrind_utils::{draw2d::draw::Draw2d, parser::parse};

pub fn criterion_benchmark(c: &mut Criterion) {
    let src = include_str!("../example.cgp");
    c.bench_function("draw2d", |b| b.iter(|| Draw2d::draw(parse(src))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
