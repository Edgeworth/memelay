use crate::generation::Generation;
use crate::{Cfg, Evaluator};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Stats<E: Evaluator> {
    pub mean_fitness: Option<E::Fitness>,
    pub num_dup: usize,
    pub mean_distance: f64,
}

pub struct RunResult<E: Evaluator> {
    pub gen: Generation<E>,
    pub stats: Option<Stats<E>>,
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

    pub fn run_iter(&mut self, compute_stats: bool) -> RunResult<E> {
        self.gen.evaluate(&self.cfg, &self.eval);
        let mut stats = None;
        if compute_stats {
            stats = Some(Stats {
                mean_fitness: self.gen.mean_fitness(),
                num_dup: self.gen.num_dup(),
                mean_distance: self.gen.mean_distance(&self.cfg, &self.eval),
            });
        }
        let mut gen = self.gen.create_next_gen(&self.cfg, &self.eval);
        std::mem::swap(&mut gen, &mut self.gen);
        RunResult { gen, stats }
    }
}
