use crate::{Evaluator, State};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::ops::Index;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct DistCache {
    cache: Vec<Vec<f64>>,
}

impl DistCache {
    pub fn new<E: Evaluator>(eval: &E, s: &[State<E::Genome>], par: bool) -> Self {
        let dist_fn =
            |i: usize| (0..s.len()).into_iter().map(|j| eval.distance(&s[i].0, &s[j].0)).collect();
        let cache = if par {
            (0..s.len()).into_par_iter().map(dist_fn).collect()
        } else {
            (0..s.len()).into_iter().map(dist_fn).collect()
        };
        Self { cache }
    }

    pub fn empty() -> Self {
        Self { cache: vec![] }
    }

    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    pub fn mean(&self) -> f64 {
        let n = (self.cache.len() * self.cache.len()) as f64;
        let sum: f64 = self.cache.iter().map(|v| v.iter().sum::<f64>()).sum();
        sum / n
    }

    pub fn max(&self) -> f64 {
        self.cache
            .iter()
            .map(|v| v.iter().fold(0.0, |a: f64, &b| a.max(b)))
            .fold(0.0, |a, b| a.max(b))
    }
}

impl Index<(usize, usize)> for DistCache {
    type Output = f64;

    fn index(&self, i: (usize, usize)) -> &f64 {
        &self.cache[i.0][i.1]
    }
}
