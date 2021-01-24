use criterion::Criterion;
use ga::cfg::{Cfg, Mutation, Niching, Species, Survival};
use ga::gen::unevaluated::UnevaluatedGen;
use ga::ops::crossover::crossover_kpx_rand;
use ga::ops::fitness::count_different;
use ga::ops::initial::rand_vec;
use ga::ops::mutation::mutate_rate;
use ga::runner::Runner;
use ga::Evaluator;
use rand::Rng;

mod common;

type State = Vec<bool>;

#[derive(Debug, Clone)]
struct Knapsack {
    max_w: f64,
    items: Vec<(f64, f64)>, // weight and value
}

impl Knapsack {
    fn new(max_w: f64, items: Vec<(f64, f64)>) -> Self {
        Self { max_w, items }
    }
}

impl Evaluator for Knapsack {
    type Genome = State;

    fn crossover(&self, s1: &mut State, s2: &mut State) {
        let mut r = rand::thread_rng();
        crossover_kpx_rand(s1, s2, 2, &mut r);
    }

    fn mutate(&self, s: &mut State, rate: f64) {
        let mut r = rand::thread_rng();
        mutate_rate(s, rate, |r| r.gen::<bool>(), &mut r);
    }

    fn fitness(&self, s: &State) -> f64 {
        let mut cur_w = 0.0;
        let mut cur_v = 0.0;
        for (i, &kept) in s.iter().enumerate() {
            let (w, v) = self.items[i];
            if kept && cur_w + w <= self.max_w {
                cur_w += w;
                cur_v += v;
            }
        }
        cur_v
    }

    fn distance(&self, s1: &State, s2: &State) -> f64 {
        count_different(s1, s2) as f64
    }
}

fn main() {
    const NUM_ITEMS: usize = 100;
    const MAX_W: f64 = 100.0;
    let base_cfg = Cfg::new(100);
    common::runner::run("knapsack", base_cfg, &|cfg| {
        let mut r = rand::thread_rng();
        let initial = rand_vec(cfg.pop_size, || rand_vec(NUM_ITEMS, || r.gen::<bool>()));
        let gen = UnevaluatedGen::initial(initial);
        let items = rand_vec(NUM_ITEMS, || {
            let w = r.gen_range(0.0..MAX_W);
            // Generate items with a narrow range of value/weight ratios, to make the
            // problem harder.
            let v = r.gen_range(0.97..1.03) * w;
            (w, v)
        });
        Runner::new(Knapsack::new(MAX_W, items), *cfg, gen)
    });
    Criterion::default().configure_from_args().final_summary();
}
