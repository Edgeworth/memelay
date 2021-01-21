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
    option_unwrap_none
)]

use rand_distr::weighted_alias::AliasableWeight;

mod prelude;
pub mod runner;
pub mod util;

pub trait Evaluator: Send + Sync + Clone {
    type T: Clone + Send + Sync;
    type F: Copy + Clone + Send + Sync + Default + PartialOrd + AliasableWeight;
    fn reproduce(&self, cfg: &Cfg, a: &Self::T, b: &Self::T) -> Self::T;
    fn fitness(&self, cfg: &Cfg, a: &Self::T) -> Self::F;
    fn distance(&self, cfg: &Cfg, a: &Self::T, b: &Self::T) -> f64;
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Cfg {
    pub xover_rate: f64,
    pub pop_size: usize,
    pub top_prop: f64,
}
