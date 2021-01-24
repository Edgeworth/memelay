use criterion::Criterion;
use ga::cfg::Cfg;
use ga::distributions::PrintableAscii;
use ga::generation::Generation;
use ga::ops::crossover::crossover_kpx_rand;
use ga::ops::fitness::count_different;
use ga::ops::initial::{rand_vec, str_to_vec};
use ga::ops::mutation::mutate_rate;
use ga::runner::Runner;
use ga::Evaluator;
use rand::Rng;

mod common;

type State = Vec<char>;

#[derive(Debug, Clone)]
struct TargetString {
    target: State,
}

impl TargetString {
    fn new(target: &str) -> Self {
        Self { target: str_to_vec(target) }
    }
}

impl Evaluator for TargetString {
    type State = State;
    type Fitness = f64;

    fn crossover(&self, _: &Cfg, s1: &mut State, s2: &mut State) {
        let mut r = rand::thread_rng();
        crossover_kpx_rand(s1, s2, 2, &mut r);
    }

    fn mutate(&self, cfg: &Cfg, s: &mut State) {
        let mut r = rand::thread_rng();
        mutate_rate(s, cfg.mutation_rate, |r| r.sample(PrintableAscii), &mut r);
    }

    fn fitness(&self, _: &Cfg, s: &State) -> f64 {
        (self.target.len() - count_different(s, &self.target)) as f64 + 1.0
    }

    fn distance(&self, _: &Cfg, s1: &State, s2: &State) -> f64 {
        count_different(s1, s2) as f64
    }
}

fn main() {
    const TARGET: &str = "Hello world!";
    let base_cfg = Cfg::new(100).with_mutation_rate(1.0 / TARGET.len() as f64);
    common::runner::run("target_string", base_cfg, &|cfg| {
        let mut r = rand::thread_rng();
        let initial = rand_vec(cfg.pop_size, || {
            rand_vec(TARGET.len(), || r.sample::<char, _>(PrintableAscii))
        });
        let gen = Generation::from_states(initial);
        Runner::new(TargetString::new(TARGET), *cfg, gen)
    });
    Criterion::default().configure_from_args().final_summary();
}
