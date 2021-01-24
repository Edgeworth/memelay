use std::ops::Index;

use crate::cfg::Cfg;
use crate::generation::Individual;
use crate::Evaluator;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct DistCache {
    cache: Vec<Vec<f64>>,
}

impl DistCache {
    pub fn new<E: Evaluator>(cfg: &Cfg, eval: &E, mems: &[Individual<E>]) -> Self {
        let n = mems.len();
        let cache = (0..n)
            .into_par_iter()
            .map(|i| {
                (0..n)
                    .into_par_iter()
                    .map(|j| eval.distance(cfg, &mems[i].state, &mems[j].state))
                    .collect()
            })
            .collect();


        Self { cache }
    }

    pub fn mean(&self) -> f64 {
        let n = (self.cache.len() * self.cache.len()) as f64;
        let sum: f64 = self.cache.par_iter().map::<_, f64>(|v| v.iter().sum()).sum();
        sum / n
    }

    pub fn max(&self) -> f64 {
        self.cache
            .par_iter()
            .map::<_, f64>(|v| v.iter().fold(0.0, |a, &b| a.max(b)))
            .reduce(|| 0.0, |a, b| a.max(b))
    }
}

impl Index<(usize, usize)> for DistCache {
    type Output = f64;

    fn index(&self, i: (usize, usize)) -> &f64 {
        &self.cache[i.0][i.1]
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Niching {
    None,
    SharedFitness(usize), // Shared fitness with a target number of species.
}
