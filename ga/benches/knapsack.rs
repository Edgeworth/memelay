use criterion::Criterion;
use ga::cfg::Cfg;
use ga::distributions::PrintableAscii;
use ga::generation::Generation;
use ga::operators::crossover::crossover_kpx_rand;
use ga::operators::fitness::count_different;
use ga::operators::mutation::mutate_iter;
use ga::runner::Runner;
use ga::Evaluator;
use rand::Rng;
use smallvec::{smallvec, SmallVec};

mod common;

type State = String;

#[derive(Debug, Clone)]
struct Knapsack {
    target: String,
}

impl Knapsack {
    fn new(target: &str) -> Self {
        Self { target: target.to_string() }
    }
}

impl Evaluator for Knapsack {
    type State = State;
    type Fitness = f64;

    fn crossover(&self, _: &Cfg, s1: &State, s2: &State) -> SmallVec<[State; 2]> {
        let mut r = rand::thread_rng();
        let (c1, c2) = crossover_kpx_rand(s1.chars(), s2.chars(), 2, &mut r);
        smallvec![c1, c2]
    }

    fn mutate(&self, cfg: &Cfg, s: &mut State) {
        let mut r = rand::thread_rng();
        *s = mutate_iter(s.chars(), cfg.mutation_rate, |r| r.sample(PrintableAscii), &mut r);
    }

    fn fitness(&self, _: &Cfg, s: &State) -> f64 {
        (self.target.len() - count_different(s.chars(), self.target.chars())) as f64 + 1.0
    }

    fn distance(&self, _: &Cfg, s1: &State, s2: &State) -> f64 {
        count_different(s1.chars(), s2.chars()) as f64
    }
}

fn main() {
    const TARGET: &str = "Hello world!";
    common::runner::run("target_string", &|cfg| {
        let mut r = rand::thread_rng();
        let initial = (0..cfg.pop_size)
            .map(|_| (0..TARGET.len()).map(|_| r.sample::<char, _>(PrintableAscii)).collect())
            .collect();
        let gen = Generation::from_states(initial);
        Runner::new(Knapsack::new(TARGET), *cfg, gen)
    });
    Criterion::default().configure_from_args().final_summary();
}
