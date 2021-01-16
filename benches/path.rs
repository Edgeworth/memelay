use criterion::{
    black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput,
};

fn compute_path() -> u128 {
    // TODO
    // shortest_path_cost_avg += PathFinder::new(
    //     &layout_cfg,
    //     &corpus,
    //     &cnst,
    //     a,
    // )
    // .path_fitness();
    0
}

fn path(_c: &mut Criterion) {
    //    c.bench_with_input(BenchmarkId::new("path_basic", size), &size, |b, &s| {
    //     b.iter(|| do_something(s));
    // });
}

criterion_group!(benches, path);
criterion_main!(benches);
