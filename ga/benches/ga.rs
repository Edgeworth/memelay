use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ga::Evaluator;
use smallvec::SmallVec;

type State = String;

#[derive(Debug, Clone)]
struct BenchEval {}

impl Evaluator for BenchEval {
    type State = State;
    type Fitness = f64;

    fn crossover(&self, _: &ga::Cfg, _s1: &State, _s2: &State) -> SmallVec<[State; 2]> {
        todo!()
    }

    fn mutate(&self, _: &ga::Cfg, _s: &mut State) {
        todo!()
    }

    fn fitness(&self, _: &ga::Cfg, _s: &State) -> f64 {
        todo!()
    }

    fn distance(&self, _: &ga::Cfg, _s1: &State, _s3: &State) -> f64 {
        todo!()
    }
}

#[inline]
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn ga(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(10))));
}

criterion_group!(benches, ga);
criterion_main!(benches);
