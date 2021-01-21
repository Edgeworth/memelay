use criterion::{criterion_group, criterion_main, Criterion};
use ga::runner::{Generation, Runner};
use ga::util::{crossover_kpx_rand, replace_rand};
use ga::{Cfg, Evaluator};
use rand::Rng;
use rand_distr::Alphanumeric;
use smallvec::smallvec;
use smallvec::SmallVec;

type State = String;

#[derive(Debug, Clone)]
struct BenchEval {
    target: String,
}

impl BenchEval {
    fn new(target: &str) -> Self {
        Self { target: target.to_string() }
    }
}

impl Evaluator for BenchEval {
    type State = State;
    type Fitness = f64;

    fn crossover(&self, _: &ga::Cfg, s1: &State, s2: &State) -> SmallVec<[State; 2]> {
        let mut r = rand::thread_rng();
        let (c1, c2) = crossover_kpx_rand(s1.chars(), s2.chars(), 2, &mut r);
        smallvec![c1, c2]
    }

    fn mutate(&self, _: &ga::Cfg, s: &mut State) {
        let mut r = rand::thread_rng();
        *s = replace_rand(s.chars(), r.sample(Alphanumeric) as char, &mut r);
    }

    fn fitness(&self, _: &ga::Cfg, _s: &State) -> f64 {
        todo!()
    }

    fn distance(&self, _: &ga::Cfg, _s1: &State, _s3: &State) -> f64 {
        todo!()
    }
}

#[inline]
fn evolve(target: &str) -> usize {
    let mut r = rand::thread_rng();
    let cfg = Cfg { crossover_rate: 0.3, pop_size: 100, top_prop: 0.1 };
    let initial = (0..target.len()).map(|_| r.sample(Alphanumeric) as char).collect();
    let gen = Generation::from_states(vec![initial]);
    let mut runner = Runner::new(BenchEval::new(target), cfg, gen);
    let mut runs = 0;
    loop {
        runs += 1;
        let best = runner.run_iter();
        println!("Generation: {} score: {:.3?}", runs, best.fitness);
        if best.state == target {
            return runs;
        }
    }
}

fn ga(c: &mut Criterion) {
    c.bench_function("hello world", |b| b.iter(|| evolve("hello world")));
}

criterion_group!(benches, ga);
criterion_main!(benches);
