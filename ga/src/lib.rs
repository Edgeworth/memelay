#![warn(rust_2018_idioms, clippy::all)]
#![feature(
    array_chunks,
    array_windows,
    bool_to_option,
    const_fn,
    destructuring_assignment,
    iterator_fold_self,
    map_first_last,
    option_result_contains,
    option_unwrap_none,
    partition_point,
    stmt_expr_attributes,
    trait_alias,
    type_alias_impl_trait
)]

use crate::cfg::Cfg;
use crate::gen::Params;
use std::fmt;

pub mod cfg;
pub mod distributions;
pub mod examples;
pub mod gen;
pub mod hyper;
pub mod ops;
pub mod runner;

pub trait Genome = fmt::Debug + Clone + Send + Sync + PartialOrd + PartialEq;
pub type State<E> = (<E as Evaluator>::Genome, Params);

pub trait FitnessFn<G: Genome> = Fn(&G) -> f64 + Sync + Send + Clone;

pub trait Evaluator: Clone + Send + Sync {
    type Genome: Genome;

    // |idx| specifies which crossover or mutation function to use. 0 is conventionally do nothing,
    // with actual crossover/mutation starting from index 1.
    fn crossover(&self, s1: &mut Self::Genome, s2: &mut Self::Genome, idx: usize);
    fn mutate(&self, s: &mut Self::Genome, rate: f64, idx: usize);
    fn fitness(&self, s: &Self::Genome) -> f64;
    fn distance(&self, s1: &Self::Genome, s2: &Self::Genome) -> f64;
}
