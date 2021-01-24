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
use num_traits::{Num, NumCast, ToPrimitive};
use std::fmt;

pub mod cfg;
pub mod distributions;
pub mod generation;
pub mod niching;
pub mod operators;
pub mod runner;

pub trait Evaluator: Send + Sync + Clone {
    type State: fmt::Debug + Clone + Send + Sync + Ord + PartialOrd + PartialEq;
    type Fitness: fmt::Debug
        + fmt::Display
        + Copy
        + Clone
        + Send
        + Sync
        + Default
        + PartialOrd
        + Num
        + NumCast
        + ToPrimitive;

    fn crossover(&self, cfg: &Cfg, s1: &mut Self::State, s2: &mut Self::State);
    // Implementations should look at Cfg::mutation_rate to mutate.
    fn mutate(&self, cfg: &Cfg, s: &mut Self::State);
    fn fitness(&self, cfg: &Cfg, s: &Self::State) -> Self::Fitness;
    fn distance(&self, cfg: &Cfg, s1: &Self::State, s2: &Self::State) -> f64;
}
