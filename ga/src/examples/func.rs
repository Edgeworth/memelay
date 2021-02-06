use crate::cfg::Cfg;
use crate::gen::unevaluated::UnevaluatedGen;
use crate::ops::crossover::crossover_arith_rand;
use crate::ops::fitness::euclidean_dist;
use crate::ops::initial::rand_vec;
use crate::ops::mutation::{mutate_normal, mutate_rate, mutate_uniform};
use crate::runner::Runner;
use crate::{Evaluator, FitnessFn};

pub type FuncState = Vec<f64>;

#[derive(Debug, Clone)]
pub struct FuncEvaluator<F: FitnessFn<FuncState>> {
    dim: usize,
    st: f64,
    en: f64,
    f: F,
}

impl<F: FitnessFn<FuncState>> FuncEvaluator<F> {
    fn new(dim: usize, st: f64, en: f64, f: F) -> Self {
        Self { dim, st, en, f }
    }
}

impl<F: FitnessFn<FuncState>> Evaluator for FuncEvaluator<F> {
    type Genome = FuncState;

    fn crossover(&self, s1: &mut FuncState, s2: &mut FuncState) {
        let mut r = rand::thread_rng();
        crossover_arith_rand(s1, s2, &mut r);
    }

    fn mutate(&self, s: &mut FuncState, rate: f64) {
        let mut r = rand::thread_rng();
        mutate_rate(s, 1.0, |v, r| mutate_normal(v, rate, r).clamp(self.st, self.en), &mut r);
    }

    fn fitness(&self, s: &FuncState) -> f64 {
        (self.f)(s)
    }

    fn distance(&self, s1: &FuncState, s2: &FuncState) -> f64 {
        euclidean_dist(s1, s2)
    }
}

pub fn func_runner(
    dim: usize,
    st: f64,
    en: f64,
    f: impl FitnessFn<FuncState>,
    cfg: &Cfg,
) -> Runner<FuncEvaluator<impl FitnessFn<FuncState>>> {
    let mut r = rand::thread_rng();
    let initial = rand_vec(cfg.pop_size, || rand_vec(dim, || mutate_uniform(st, en, &mut r)));
    let gen = UnevaluatedGen::initial(initial);
    Runner::new(FuncEvaluator::new(dim, st, en, f), *cfg, gen)
}
