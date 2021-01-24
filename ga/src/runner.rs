use crate::generation::Generation;
use crate::{Cfg, Evaluator};
use derive_more::Display;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Stats<E: Evaluator> {
    pub best_fitness: E::Fitness,
    pub mean_fitness: E::Fitness,
    pub num_dup: usize,
    pub mean_distance: f64,
}

#[derive(Debug, Display, Clone, PartialEq)]
#[display(fmt = "Run({})", gen)]
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
                best_fitness: self.gen.best().fitness,
                mean_fitness: self.gen.mean_fitness(),
                num_dup: self.gen.num_dup(),
                mean_distance: self.gen.dists(&self.cfg, &self.eval).mean(),
            });
        }
        let mut gen = self.gen.create_next_gen(&self.cfg, &self.eval);
        std::mem::swap(&mut gen, &mut self.gen);
        RunResult { gen, stats }
    }
}
