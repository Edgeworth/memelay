use crate::ga::runner::Ctx;

pub mod generation;
pub mod runner;
pub mod util;

pub trait Evaluator {
    type T;
    type F: PartialOrd;
    fn reproduce(&mut self, ctx: &Ctx, a: &Self::T, b: &Self::T) -> Self::T;
    fn fitness(&mut self, ctx: &Ctx, a: &Self::T) -> Self::F;
    fn distance(&mut self, ctx: &Ctx, a: &Self::T, b: &Self::T) -> f64;
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Cfg {
    pub xover_rate: f64,
    pub pop_size: usize,
}
