use crate::ga::{Cfg, Evaluator};

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Individual<E: Evaluator> {
    pub state: E::T,
    pub fitness: E::F,
    pub species: usize,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Ctx {
    pub xover_rate: f64,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Runner<E: Evaluator> {
    eval: E,
    cfg: Cfg,
}

impl<E: Evaluator> Runner<E> {
    pub fn new(eval: E, cfg: Cfg) -> Self {
        Self { eval, cfg }
    }

    pub fn run_iter(&mut self) -> Individual<E> {
        todo!()
    }

    fn evalute(&mut self) {}
}
