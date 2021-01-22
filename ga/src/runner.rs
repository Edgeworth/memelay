use crate::generation::Generation;
use crate::{Cfg, Evaluator};

pub struct RunResult<E: Evaluator> {
    pub gen: Generation<E>,
}

pub struct Runner<E: Evaluator> {
    eval: E,
    cfg: Cfg,
    gen: Generation<E>,
}

impl<E: Evaluator> Runner<E> {
    pub fn new(eval: E, cfg: Cfg, gen: Generation<E>) -> Self {
        Self { eval, cfg, gen }
    }

    pub fn run_iter(&mut self) -> RunResult<E> {
        self.gen.evaluate(&self.cfg, &self.eval);
        let mut gen = self.gen.create_next_gen(&self.eval, &self.cfg);
        std::mem::swap(&mut gen, &mut self.gen);
        RunResult { gen }
    }
}
