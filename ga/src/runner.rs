use crate::{Cfg, Evaluator};
use num_traits::NumCast;
use rand::Rng;
use rand_distr::{Distribution, WeightedAliasIndex};
use rayon::prelude::*;
use smallvec::smallvec;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Individual<E: Evaluator> {
    pub state: E::State,
    pub fitness: E::Fitness,
    pub species: usize,
}

pub struct Generation<E: Evaluator> {
    mems: Vec<Individual<E>>,
}

impl<E: Evaluator> Generation<E> {
    pub fn from_states(states: Vec<E::State>) -> Self {
        let mems = states
            .into_iter()
            .map(|state| Individual { state, fitness: Default::default(), species: 0 })
            .collect();
        Self { mems }
    }

    pub fn get_best(&self) -> Individual<E> {
        self.mems[0].clone()
    }

    pub fn mean_fitness(&self) -> Option<E::Fitness> {
        Some(
            self.mems.iter().map(|v| v.fitness).fold_first(|a, b| a + b)?
                / NumCast::from(self.mems.len())?,
        )
    }

    fn evaluate(&mut self, cfg: &Cfg, eval: &E) {
        self.mems.par_iter_mut().for_each(|mem| {
            let f = eval.fitness(cfg, &mem.state);
            mem.fitness = f;
        });
        self.mems.par_sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
    }

    fn get_top_proportion(&self, prop: f64) -> Vec<Individual<E>> {
        let num = self.mems.len() as f64 * prop;
        self.mems.iter().cloned().take(num as usize).collect()
    }

    fn create_next_gen(&self, eval: &E, cfg: &Cfg) -> Generation<E> {
        // Pick survivors:
        let survivors = self.get_top_proportion(cfg.top_prop);

        // Reproduce:
        let mut new_states = (survivors.len()..self.mems.len())
            .into_par_iter()
            .map(|_| {
                let mut r = rand::thread_rng();
                let idx =
                    WeightedAliasIndex::new(self.mems.iter().map(|mem| mem.fitness).collect())
                        .unwrap();
                let a = &self.mems[idx.sample(&mut r)];
                if r.gen::<f64>() < cfg.crossover_rate {
                    let b = &self.mems[idx.sample(&mut r)];
                    eval.crossover(cfg, &a.state, &b.state).into_iter()
                } else {
                    let mut s = a.state.clone();
                    eval.mutate(cfg, &mut s);
                    smallvec![s].into_iter()
                }
            })
            .flatten_iter()
            .collect::<Vec<_>>();
        new_states.extend(survivors.into_iter().map(|mem| mem.state));
        Self::from_states(new_states)
    }
}

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
