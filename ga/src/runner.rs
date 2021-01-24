use crate::gen::evaluated::EvaluatedGen;
use crate::gen::unevaluated::UnevaluatedGen;
use crate::{Cfg, Evaluator};
use derive_more::Display;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Stats {
    pub best_fitness: f64,
    pub mean_fitness: f64,
    pub num_dup: usize,
    pub mean_distance: f64,
}

#[derive(Debug, Display, Clone, PartialEq)]
#[display(fmt = "Run({})", gen)]
pub struct RunResult<E: Evaluator> {
    pub gen: EvaluatedGen<E>,
    pub stats: Option<Stats>,
}

pub struct Runner<E: Evaluator> {
    eval: E,
    cfg: Cfg,
    gen: UnevaluatedGen<E>,
}

impl<E: Evaluator> Runner<E> {
    pub fn new(eval: E, cfg: Cfg, gen: UnevaluatedGen<E>) -> Self {
        Self { eval, cfg, gen }
    }

    pub fn run_iter(&mut self, compute_stats: bool) -> RunResult<E> {
        let mut evaluated = self.gen.evaluate(&self.cfg, &self.eval);
        let mut stats = None;
        if compute_stats {
            stats = Some(Stats {
                best_fitness: evaluated.best().base_fitness,
                mean_fitness: evaluated.mean_base_fitness(),
                num_dup: evaluated.num_dup(),
                mean_distance: evaluated.dists(&self.cfg, &self.eval).mean(),
            });
        }
        let mut gen = evaluated.next_gen(&self.cfg, &self.eval);
        std::mem::swap(&mut gen, &mut self.gen);
        RunResult { gen: evaluated, stats }
    }
}
