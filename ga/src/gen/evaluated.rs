use crate::cfg::{Selection, Survival};
use crate::gen::species::DistCache;
use crate::gen::unevaluated::UnevaluatedGen;
use crate::ops::sampling::{multi_rws, sus};
use crate::{Cfg, Evaluator};
use derive_more::Display;
use rand::Rng;
use rayon::prelude::*;

#[derive(Debug, Display, Clone, PartialOrd, PartialEq)]
#[display(fmt = "state {:?}, fitness {}", state, fitness)]
pub struct Member<E: Evaluator> {
    pub state: E::State,
    pub fitness: f64,
    pub species: i32,
}

#[derive(Debug, Display, Clone, PartialOrd, PartialEq)]
#[display(fmt = "pop: {}, best: {}", "mems.len()", "self.best()")]
pub struct EvaluatedGen<E: Evaluator> {
    mems: Vec<Member<E>>,
    cache: Option<DistCache>,
}

impl<E: Evaluator> EvaluatedGen<E> {
    pub fn new(mut mems: Vec<Member<E>>) -> Self {
        mems.par_sort_unstable_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        Self { mems, cache: None }
    }

    pub fn best(&self) -> Member<E> {
        self.mems[0].clone()
    }

    pub fn mean_fitness(&self) -> f64 {
        self.mems.iter().map(|v| v.fitness).sum::<f64>() / self.mems.len() as f64
    }

    pub fn num_dup(&self) -> usize {
        let mut mems_copy = self.mems.iter().map(|v| &v.state).cloned().collect::<Vec<_>>();
        mems_copy.par_sort_unstable();
        mems_copy.dedup();
        self.mems.len() - mems_copy.len()
    }

    pub fn dists(&mut self, cfg: &Cfg, eval: &E) -> &DistCache {
        if self.cache.is_none() {
            let states = self.mems.iter().map(|mem| mem.state.clone()).collect::<Vec<_>>();
            self.cache = Some(DistCache::new(cfg, eval, &states));
        }
        self.cache.as_ref().unwrap()
    }

    fn survivors(&self, survival: Survival) -> Vec<E::State> {
        match survival {
            Survival::TopProportion(prop) => {
                let num = self.mems.len() as f64 * prop;
                self.mems.iter().map(|v| &v.state).cloned().take(num as usize).collect()
            }
            Survival::SpeciesTopProportion(_) => todo!(),
        }
    }

    fn selection(&self, cfg: &Cfg) -> (E::State, E::State) {
        let mut r = rand::thread_rng();
        let fitnesses = self.mems.iter().map(|v| v.fitness).collect::<Vec<_>>();
        let idxs = match cfg.selection {
            Selection::Sus => sus(&fitnesses, 2, &mut r),
            Selection::Roulette => multi_rws(&fitnesses, 2, &mut r),
        };
        (self.mems[idxs[0]].state.clone(), self.mems[idxs[1]].state.clone())
    }

    pub fn next_gen(&self, cfg: &Cfg, eval: &E) -> UnevaluatedGen<E> {
        // Pick survivors:
        let mut new_states = self.survivors(cfg.survival);
        let remaining = cfg.pop_size - new_states.len();
        // Reproduce:
        let reproduced = (0..remaining)
            .into_par_iter()
            .map(|_| {
                let mut r = rand::thread_rng();
                let (mut a, mut b) = self.selection(cfg);
                if r.gen::<f64>() < cfg.crossover_rate {
                    eval.crossover(cfg, &mut a, &mut b);
                }
                eval.mutate(cfg, &mut a);
                eval.mutate(cfg, &mut b);
                vec![a, b].into_iter()
            })
            .flatten_iter()
            .collect::<Vec<_>>();
        new_states.extend(reproduced.into_iter().take(remaining));
        UnevaluatedGen::from_states(new_states)
    }
}
