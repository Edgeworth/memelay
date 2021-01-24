use crate::cfg::{Crossover, Mutation, Selection, Survival};
use crate::gen::species::DistCache;
use crate::gen::unevaluated::UnevaluatedGen;
use crate::ops::mutation::mutate_lognorm;
use crate::ops::sampling::{multi_rws, sus};
use crate::{Cfg, Evaluator, State};
use derive_more::Display;
use rand::Rng;
use rayon::prelude::*;

#[derive(Debug, Display, Clone, PartialOrd, PartialEq)]
#[display(fmt = "base fitness {:.2}, selection fitness {:.2}", base_fitness, selection_fitness)]
pub struct Member<E: Evaluator> {
    pub state: State<E>,
    pub base_fitness: f64, // Original fitness, generated by Evaluator fitness function.
    pub selection_fitness: f64, // Potentially adjusted fitness, for selection.
    pub species: usize,
}

#[derive(Debug, Display, Clone, PartialOrd, PartialEq)]
#[display(fmt = "pop: {}, best: {}", "mems.len()", "self.best()")]
pub struct EvaluatedGen<E: Evaluator> {
    mems: Vec<Member<E>>,
    cache: Option<DistCache>,
}

impl<E: Evaluator> EvaluatedGen<E> {
    pub fn new(mut mems: Vec<Member<E>>) -> Self {
        mems.par_sort_unstable_by(|a, b| {
            b.selection_fitness.partial_cmp(&a.selection_fitness).unwrap()
        });
        Self { mems, cache: None }
    }

    pub fn best(&self) -> Member<E> {
        self.mems[0].clone()
    }

    pub fn mean_base_fitness(&self) -> f64 {
        self.mems.iter().map(|v| v.base_fitness).sum::<f64>() / self.mems.len() as f64
    }

    pub fn num_dup(&self) -> usize {
        let mut mems_copy = self.mems.iter().map(|v| &v.state.0).cloned().collect::<Vec<_>>();
        mems_copy.par_sort_unstable();
        mems_copy.dedup();
        self.mems.len() - mems_copy.len()
    }

    pub fn num_species(&self) -> usize {
        // Relies on species index assignment to be contigous from zero.
        self.mems.iter().map(|mem| mem.species).max().unwrap_or(0) + 1
    }

    pub fn dists(&mut self, eval: &E) -> &DistCache {
        if self.cache.is_none() {
            let states = self.mems.iter().map(|mem| mem.state.clone()).collect::<Vec<_>>();
            self.cache = Some(DistCache::new(eval, &states));
        }
        self.cache.as_ref().unwrap()
    }

    fn take_proportion(mems: &[Member<E>], prop: f64) -> Vec<State<E>> {
        let num = mems.len() as f64 * prop;
        mems.iter().map(|v| &v.state).cloned().take(num as usize).collect()
    }

    fn survivors(&self, survival: Survival) -> Vec<State<E>> {
        match survival {
            Survival::TopProportion(prop) => Self::take_proportion(&self.mems, prop),
            Survival::SpeciesTopProportion(prop) => {
                let mut by_species = vec![Vec::new(); self.num_species()];
                self.mems.iter().for_each(|mem| by_species[mem.species].push(mem.clone()));
                by_species.into_iter().map(|v| Self::take_proportion(&v, prop)).flatten().collect()
            }
        }
    }

    fn selection(&self, selection: Selection) -> [State<E>; 2] {
        let mut r = rand::thread_rng();
        let fitnesses = self.mems.iter().map(|v| v.selection_fitness).collect::<Vec<_>>();
        let idxs = match selection {
            Selection::Sus => sus(&fitnesses, 2, &mut r),
            Selection::Roulette => multi_rws(&fitnesses, 2, &mut r),
        };
        [self.mems[idxs[0]].state.clone(), self.mems[idxs[1]].state.clone()]
    }

    fn crossover(&self, crossover: Crossover, eval: &E, s1: &mut State<E>, s2: &mut State<E>) {
        let mut r = rand::thread_rng();
        match crossover {
            Crossover::Fixed(rate) => {
                s1.1.crossover_rate = rate;
                s2.1.crossover_rate = rate;
            }
            Crossover::Adaptive(lrate) => {
                // Just mutate the crossover rates.
                s1.1.crossover_rate =
                    mutate_lognorm(s1.1.crossover_rate, lrate, &mut r).clamp(0.0, 1.0);
                s2.1.crossover_rate =
                    mutate_lognorm(s2.1.crossover_rate, lrate, &mut r).clamp(0.0, 1.0);
            }
        };
        if 2.0 * r.gen::<f64>() < s1.1.crossover_rate + s2.1.crossover_rate {
            eval.crossover(&mut s1.0, &mut s2.0);
        }
    }

    fn mutation(&self, mutation: Mutation, eval: &E, s: &mut State<E>) {
        let mut r = rand::thread_rng();
        match mutation {
            Mutation::Fixed(rate) => s.1.mutation_rate = rate,
            Mutation::Adaptive(lrate) => {
                s.1.mutation_rate =
                    mutate_lognorm(s.1.mutation_rate, lrate, &mut r).clamp(0.0, 1.0);
            }
        };
        eval.mutate(&mut s.0, s.1.mutation_rate);
    }


    pub fn next_gen(&self, cfg: &Cfg, eval: &E) -> UnevaluatedGen<E> {
        // Pick survivors:
        let mut new_states = self.survivors(cfg.survival);
        let remaining = cfg.pop_size - new_states.len();
        // Reproduce:
        let reproduced = (0..remaining)
            .into_par_iter()
            .map(|_| {
                let [mut s1, mut s2] = self.selection(cfg.selection);
                self.crossover(cfg.crossover, eval, &mut s1, &mut s2);
                self.mutation(cfg.mutation, eval, &mut s1);
                self.mutation(cfg.mutation, eval, &mut s2);
                vec![s1, s2].into_iter()
            })
            .flatten_iter()
            .collect::<Vec<_>>();
        new_states.extend(reproduced.into_iter().take(remaining));
        UnevaluatedGen::new(new_states)
    }
}
