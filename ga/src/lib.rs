#![warn(rust_2018_idioms, clippy::all)]
#![feature(
    const_fn,
    option_result_contains,
    trait_alias,
    iterator_fold_self,
    type_alias_impl_trait,
    partition_point,
    bool_to_option,
    map_first_last,
    option_unwrap_none,
    array_windows
)]

use rand_distr::weighted_alias::AliasableWeight;
use smallvec::SmallVec;

pub mod runner;
pub mod util;

pub trait Evaluator: Send + Sync + Clone {
    type State: Clone + Send + Sync;
    type Fitness: Copy + Clone + Send + Sync + Default + PartialOrd + AliasableWeight;

    fn crossover(
        &self,
        cfg: &Cfg,
        s1: &Self::State,
        s2: &Self::State,
    ) -> SmallVec<[Self::State; 2]>;
    fn mutate(&self, cfg: &Cfg, s: &mut Self::State);
    fn fitness(&self, cfg: &Cfg, s: &Self::State) -> Self::Fitness;
    fn distance(&self, cfg: &Cfg, s1: &Self::State, s2: &Self::State) -> f64;
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Cfg {
    pub xover_rate: f64,
    pub pop_size: usize,
    pub top_prop: f64,
}
