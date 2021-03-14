use criterion::{criterion_group, criterion_main, Criterion};
use ga::cfg::{Cfg, Crossover, Mutation, Niching, Selection, Species, Survival};
use ga::examples::ackley::ackley_runner;
use ga::examples::griewank::griewank_runner;
use ga::examples::knapsack::knapsack_runner;
use ga::examples::rastrigin::rastrigin_runner;
use ga::examples::target_string::target_string_runner;

fn get_cfg() -> Cfg {
    Cfg::new(100)
        .with_mutation(Mutation::Adaptive)
        .with_crossover(Crossover::Adaptive)
        .with_survival(Survival::TopProportion(0.25))
        .with_selection(Selection::Sus)
        .with_species(Species::None)
        .with_niching(Niching::None)
}

fn rastrigin(c: &mut Criterion) {
    c.bench_function("rastrigin", |b| {
        let mut r = rastrigin_runner(2, get_cfg());
        b.iter(|| r.run_iter())
    });
}

fn griewank(c: &mut Criterion) {
    c.bench_function("griewank", |b| {
        let mut r = griewank_runner(2, get_cfg());
        b.iter(|| r.run_iter())
    });
}

fn ackley(c: &mut Criterion) {
    c.bench_function("ackley", |b| {
        let mut r = ackley_runner(2, get_cfg());
        b.iter(|| r.run_iter())
    });
}

fn knapsack(c: &mut Criterion) {
    c.bench_function("knapsack", |b| {
        let mut r = knapsack_runner(get_cfg());
        b.iter(|| r.run_iter())
    });
}

fn target_string(c: &mut Criterion) {
    c.bench_function("target_string", |b| {
        let mut r = target_string_runner(get_cfg());
        b.iter(|| r.run_iter())
    });
}

criterion_group!(benches, rastrigin, griewank, ackley, knapsack, target_string);
criterion_main!(benches);
