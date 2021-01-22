use crate::util::sus;
use crate::{Cfg, Evaluator};
use num_traits::NumCast;
use rand::Rng;
use rayon::prelude::*;
use smallvec::smallvec;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SelectionMethod {
    StochasticUniformSampling,
    RouletteWheel,
}

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

    pub fn best(&self) -> Individual<E> {
        self.mems[0].clone()
    }

    pub fn mean_fitness(&self) -> Option<E::Fitness> {
        Some(
            self.mems.iter().map(|v| v.fitness).fold_first(|a, b| a + b)?
                / NumCast::from(self.mems.len())?,
        )
    }

    pub fn individuals(&self) -> &[Individual<E>] {
        &self.mems
    }

    pub fn evaluate(&mut self, cfg: &Cfg, eval: &E) {
        self.mems.par_iter_mut().for_each(|mem| {
            let f = eval.fitness(cfg, &mem.state);
            mem.fitness = f;
        });
        self.mems.par_sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
    }

    fn get_top_proportion(&self, prop: f64) -> Vec<E::State> {
        let num = self.mems.len() as f64 * prop;
        self.mems.iter().map(|v| &v.state).cloned().take(num as usize).collect()
    }

    pub fn create_next_gen(&self, eval: &E, cfg: &Cfg) -> Generation<E> {
        // Pick survivors:
        let mut new_states = self.get_top_proportion(cfg.top_prop);
        let remaining = cfg.pop_size - new_states.len();
        // Reproduce:
        let reproduced = (0..remaining)
            .into_par_iter()
            .map(|_| {
                let mut r = rand::thread_rng();
                let idxs = sus(self.mems.iter().map(|v| &v.fitness), 2, &mut r);
                let a = &self.mems[idxs[0]];
                if r.gen::<f64>() < cfg.crossover_rate {
                    let b = &self.mems[idxs[1]];
                    eval.crossover(cfg, &a.state, &b.state).into_iter()
                } else {
                    let mut s = a.state.clone();
                    eval.mutate(cfg, &mut s);
                    smallvec![s].into_iter()
                }
            })
            .flatten_iter()
            .collect::<Vec<_>>();
        new_states.extend(reproduced.into_iter().take(remaining));
        Self::from_states(new_states)
    }
}
