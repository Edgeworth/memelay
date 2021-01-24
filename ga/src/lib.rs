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
    array_windows,
    array_chunks
)]

use crate::cfg::Cfg;
use std::fmt;

pub mod cfg;
pub mod distributions;
pub mod gen;
pub mod ops;
pub mod runner;

pub trait Evaluator: Send + Sync + Clone {
    type State: fmt::Debug + Clone + Send + Sync + Ord + PartialOrd + PartialEq;

    fn crossover(&self, s1: &mut Self::State, s2: &mut Self::State);
    // Implementations should look at Cfg::mutation_rate to mutate.
    fn mutate(&self, s: &mut Self::State, rate: f64);
    fn fitness(&self, s: &Self::State) -> f64;
    fn distance(&self, s1: &Self::State, s2: &Self::State) -> f64;
}
