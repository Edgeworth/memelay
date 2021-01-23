use criterion::Criterion;
use ga::cfg::Cfg;
use ga::generation::Generation;
use ga::runner::Runner;
use ga::Evaluator;

mod common;

type State = Vec<bool>;

#[derive(Debug, Clone)]
struct Knapsack {}

impl Knapsack {
    fn new() -> Self {
        Self {}
    }
}

impl Evaluator for Knapsack {
    type State = State;
    type Fitness = f64;

    fn crossover(&self, _: &Cfg, _s1: &mut State, _s2: &mut State) {}

    fn mutate(&self, _cfg: &Cfg, _s: &mut State) {}

    fn fitness(&self, _: &Cfg, _s: &State) -> f64 {
        0.0
    }

    fn distance(&self, _: &Cfg, _s1: &State, _s2: &State) -> f64 {
        0.0
    }
}

fn main() {
    common::runner::run("knapsack", &|cfg| {
        let _r = rand::thread_rng();
        let gen = Generation::from_states(vec![]);
        Runner::new(Knapsack::new(), *cfg, gen)
    });
    Criterion::default().configure_from_args().final_summary();
}
