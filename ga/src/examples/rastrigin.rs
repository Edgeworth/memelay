use crate::cfg::Cfg;
use crate::gen::unevaluated::UnevaluatedGen;
use crate::ops::crossover::crossover_kpx_rand;
use crate::ops::fitness::{count_different, euclidean_dist};
use crate::ops::initial::rand_vec;
use crate::ops::mutation::{mutate_normal, mutate_rate, mutate_uniform};
use crate::runner::Runner;
use crate::Evaluator;
use std::f64::consts::PI;
use std::ops::Range;

type State = Vec<f64>;
const RANGE: Range<f64> = -5.12..5.12;

#[derive(Debug, Clone)]
pub struct Rastrigin {
    dim: usize,
}

impl Rastrigin {
    fn new(dim: usize) -> Self {
        Self { dim }
    }
}

impl Evaluator for Rastrigin {
    type Genome = State;

    fn crossover(&self, s1: &mut State, s2: &mut State) {
        let mut r = rand::thread_rng();
        crossover_kpx_rand(s1, s2, 2, &mut r);
    }

    fn mutate(&self, s: &mut State, rate: f64) {
        let mut r = rand::thread_rng();
        mutate_rate(s, 1.0, |v, r| mutate_normal(v, rate, r), &mut r);
    }

    fn fitness(&self, s: &State) -> f64 {
        const A: f64 = 10.0;
        let mut v = A * self.dim as f64;
        for &x in s.iter() {
            v += x * x - A * (2.0 * PI * x).cos();
            v = v.clamp(RANGE.start, RANGE.end);
        }
        // Convert to a maximisation problem
        1.0 / (1.0 + v)
    }

    fn distance(&self, s1: &State, s2: &State) -> f64 {
        euclidean_dist(s1, s2)
    }
}

pub fn rastrigin(dim: usize, cfg: &Cfg) -> Runner<Rastrigin> {
    let mut r = rand::thread_rng();
    let initial = rand_vec(cfg.pop_size, || rand_vec(dim, || mutate_uniform(RANGE, &mut r)));
    let gen = UnevaluatedGen::initial(initial);
    Runner::new(Rastrigin::new(dim), *cfg, gen)
}
