use criterion::{criterion_group, criterion_main, Criterion};

fn path(_c: &mut Criterion) {}

criterion_group!(benches, path);
criterion_main!(benches);
