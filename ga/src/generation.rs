use crate::niching::{DistCache, Niching};
use crate::ops::sampling::{multi_rws, sus};
use crate::{Cfg, Evaluator};
use derive_more::Display;
use num_traits::NumCast;
use rand::Rng;
use rayon::prelude::*;
use std::cmp::Ordering;

const EP: f64 = 1.0e-6;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Selection {
    Sus,
    Roulette,
}

#[derive(Debug, Display, Clone, PartialOrd, PartialEq)]
#[display(fmt = "state {:?}, fitness {}", state, fitness)]
pub struct Member<E: Evaluator> {
    pub state: E::State,
    pub fitness: E::Fitness,
    pub species: usize,
}

#[derive(Debug, Display, Clone, PartialOrd, PartialEq)]
#[display(fmt = "pop: {}, best: {}", "mems.len()", "self.best()")]
pub struct Generation<E: Evaluator> {
    mems: Vec<Member<E>>,
    cache: Option<DistCache>,
}

impl<E: Evaluator> Generation<E> {
    pub fn from_states(states: Vec<E::State>) -> Self {
        if states.is_empty() {
            panic!("Generation must not be empty");
        }
        let mems = states
            .into_iter()
            .map(|state| Member { state, fitness: Default::default(), species: 0 })
            .collect();
        Self { mems, cache: None }
    }

    pub fn best(&self) -> Member<E> {
        self.mems[0].clone()
    }

    pub fn mean_fitness(&self) -> E::Fitness {
        self.mems.iter().map(|v| v.fitness).fold_first(|a, b| a + b).unwrap()
            / NumCast::from(self.mems.len()).unwrap()
    }

    pub fn dists(&mut self, cfg: &Cfg, eval: &E) -> &DistCache {
        if self.cache.is_none() {
            self.cache = Some(DistCache::new(cfg, eval, &self.mems));
        }
        self.cache.as_ref().unwrap()
    }


    fn assign_species(&mut self, cfg: &Cfg, eval: &E, dist: f64) -> usize {
        let n = self.mems.len();
        for v in self.mems.iter_mut() {
            v.species = 0;
        }
        let mut num_species = 0;
        for i in 0..n {
            if self.mems[i].species == 0 {
                continue;
            }
            self.mems[i].species = num_species;
            for j in (i + 1)..n {
                if self.dists(cfg, eval)[(i, j)] <= dist {
                    self.mems[j].species = num_species;
                }
            }
            num_species += 1;
        }
        num_species
    }

    pub fn num_dup(&self) -> usize {
        let mut mems_copy = self.mems.iter().map(|v| &v.state).cloned().collect::<Vec<_>>();
        mems_copy.par_sort_unstable();
        mems_copy.dedup();
        self.mems.len() - mems_copy.len()
    }

    pub fn mems(&self) -> &[Member<E>] {
        &self.mems
    }

    pub fn fitness(&self, cfg: &Cfg, eval: &E, mem: &Member<E>) -> E::Fitness {
        // TODO: shared fitness here.
        eval.fitness(cfg, &mem.state)
    }

    pub fn evaluate(&mut self, cfg: &Cfg, eval: &E) {
        // Speciate if necessary.
        match cfg.niching {
            Niching::None => {}
            Niching::SharedFitness(target) => {
                let mut lo = 0.0;
                let mut hi = self.dists(cfg, eval).max();
                while hi - lo > EP {
                    let mid = (lo + hi) / 2.0;
                    match self.assign_species(cfg, eval, mid).cmp(&target) {
                        Ordering::Less => hi = mid,
                        Ordering::Equal => break,
                        Ordering::Greater => lo = mid,
                    }
                }
            }
        };
        let mut mems = self.mems.clone();
        mems.par_iter_mut().for_each(|mem| {
            mem.fitness = self.fitness(cfg, eval, mem);
        });
        mems.par_sort_unstable_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        self.mems = mems;
    }

    fn get_top_proportion(&self, prop: f64) -> Vec<E::State> {
        let num = self.mems.len() as f64 * prop;
        self.mems.iter().map(|v| &v.state).cloned().take(num as usize).collect()
    }

    fn selection(&self, cfg: &Cfg) -> (E::State, E::State) {
        let mut r = rand::thread_rng();
        let fitnesses = self.mems.iter().map(|v| &v.fitness);
        let idxs = match cfg.selection {
            Selection::Sus => sus(fitnesses, 2, &mut r),
            Selection::Roulette => multi_rws(fitnesses, 2, &mut r),
        };
        (self.mems[idxs[0]].state.clone(), self.mems[idxs[1]].state.clone())
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
        Self::from_states(new_states)
    }
}
