use criterion::{criterion_group, criterion_main, Criterion};
use ga::cfg::{Cfg, Crossover, Mutation, Niching, Selection, Species, Survival};
use ga::examples::rastrigin::rastrigin_runner;

fn rastringin(c: &mut Criterion) {
    c.bench_function("rastringin", |b| {
        let cfg = Cfg::new(100)
            .with_mutation(Mutation::Fixed(vec![0.9, 0.1]))
            .with_crossover(Crossover::Fixed(vec![0.3, 0.7]))
            .with_survival(Survival::TopProportion(0.25))
            .with_selection(Selection::Sus)
            .with_species(Species::None)
            .with_niching(Niching::None);
        let mut r = rastrigin_runner(2, cfg);
        b.iter(|| r.run_iter())
    });
}

criterion_group!(benches, rastringin);
criterion_main!(benches);
