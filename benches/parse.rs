use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use cygrind_utils::parser::parse;

pub fn criterion_benchmark(c: &mut Criterion) {
    let src = include_str!("../example.cgp");
    let mut g = c.benchmark_group("Parse");
    g.throughput(Throughput::Bytes(src.len() as u64));
    g.bench_function("parse", |b| b.iter(|| parse(src)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
