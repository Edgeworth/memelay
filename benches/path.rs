use criterion::{Criterion, criterion_group, criterion_main};

fn path(_c: &mut Criterion) {}

criterion_group!(benches, path);
criterion_main!(benches);
