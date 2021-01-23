use crate::util::{multi_rws, sus};
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

    pub fn mean_distance(&self, cfg: &Cfg, eval: &E) -> f64 {
        let mut dist = 0.0;
        let n = self.mems.len();
        for i in 0..n {
            for j in (i + 1)..n {
                dist += eval.distance(cfg, &self.mems[i].state, &self.mems[j].state);
            }
        }
        2.0 * dist / (n * (n - 1)) as f64
    }

    pub fn num_dup(&self) -> usize {
        let mut mems_copy = self.mems.iter().map(|v| &v.state).cloned().collect::<Vec<_>>();
        mems_copy.par_sort_unstable();
        mems_copy.dedup();
        self.mems.len() - mems_copy.len()
    }

    pub fn individuals(&self) -> &[Individual<E>] {
        &self.mems
    }

    pub fn evaluate(&mut self, cfg: &Cfg, eval: &E) {
        self.mems.par_iter_mut().for_each(|mem| {
            let f = eval.fitness(cfg, &mem.state);
            mem.fitness = f;
        });
        self.mems.par_sort_unstable_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
    }

    fn get_top_proportion(&self, prop: f64) -> Vec<E::State> {
        let num = self.mems.len() as f64 * prop;
        self.mems.iter().map(|v| &v.state).cloned().take(num as usize).collect()
    }

    fn selection(&self, cfg: &Cfg) -> Vec<usize> {
        let mut r = rand::thread_rng();
        let fitnesses = self.mems.iter().map(|v| &v.fitness);
        match cfg.selection_method {
            SelectionMethod::StochasticUniformSampling => sus(fitnesses, 2, &mut r),
            SelectionMethod::RouletteWheel => multi_rws(fitnesses, 2, &mut r),
        }
    }

    pub fn create_next_gen(&self, cfg: &Cfg, eval: &E) -> Generation<E> {
        // Pick survivors:
        let mut new_states = self.get_top_proportion(cfg.top_prop);
        let remaining = cfg.pop_size - new_states.len();
        // Reproduce:
        let reproduced = (0..remaining)
            .into_par_iter()
            .map(|_| {
                let mut r = rand::thread_rng();
                let idxs = self.selection(cfg);
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
