use crate::cfg::{Crossover, Mutation, Selection, Survival};
use crate::gen::species::DistCache;
use crate::gen::unevaluated::UnevaluatedGen;
use crate::ops::crossover::crossover_blx;
use crate::ops::mutation::{mutate_lognorm, mutate_rate};
use crate::ops::sampling::{multi_rws, rws, sus};
use crate::ops::util::clamp_vec;
use crate::{Cfg, Evaluator, State};
use derive_more::Display;
use eyre::{eyre, Result};
use rayon::prelude::*;

#[derive(Display, Clone, PartialOrd, PartialEq)]
#[display(fmt = "base fitness {:.2}, selection fitness {:.2}", base_fitness, selection_fitness)]
pub struct Member<E: Evaluator> {
    pub state: State<E>,
    pub base_fitness: f64, // Original fitness, generated by Evaluator fitness function.
    pub selection_fitness: f64, // Potentially adjusted fitness, for selection.
    pub species: usize,
}

#[derive(Display, Clone, PartialOrd, PartialEq)]
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

    pub fn size(&self) -> usize {
        self.mems.len()
    }

    pub fn best(&self) -> Member<E> {
        self.mems[0].clone()
    }

    pub fn mean_base_fitness(&self) -> f64 {
        self.mems.iter().map(|v| v.base_fitness).sum::<f64>() / self.mems.len() as f64
    }

    pub fn num_dup(&self) -> usize {
        let mut mems_copy = self.mems.iter().map(|v| &v.state.0).cloned().collect::<Vec<_>>();
        mems_copy.par_sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
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
        let fitnesses = self.mems.iter().map(|v| v.selection_fitness).collect::<Vec<_>>();
        let idxs = match selection {
            Selection::Sus => sus(&fitnesses, 2),
            Selection::Roulette => multi_rws(&fitnesses, 2),
        };
        [self.mems[idxs[0]].state.clone(), self.mems[idxs[1]].state.clone()]
    }

    fn check_weights(weights: &[f64], l: usize) -> Result<()> {
        if weights.len() != l {
            return Err(eyre!("number of fixed weights {} doesn't match {}", weights.len(), l));
        }
        for &v in weights.iter() {
            if v < 0.0 {
                return Err(eyre!("weights must all be non-negative: {}", v));
            }
        }
        Ok(())
    }

    fn crossover(
        &self,
        crossover: &Crossover,
        eval: &E,
        s1: &mut State<E>,
        s2: &mut State<E>,
    ) -> Result<()> {
        match crossover {
            Crossover::Fixed(rates) => {
                s1.1.crossover = rates.clone();
                s2.1.crossover = rates.clone();
            }
            Crossover::Adaptive => {
                // We need to generate one crossover rate vector from two parents.
                // Use blend crossover to do this, and take the first one.
                crossover_blx(&mut s1.1.crossover, &mut s2.1.crossover, 0.5);
                clamp_vec(&mut s1.1.crossover, Some(0.0), None);
            }
        };
        Self::check_weights(&s1.1.crossover, E::NUM_CROSSOVER)?;
        Self::check_weights(&s2.1.crossover, E::NUM_CROSSOVER)?;
        let idx = rws(&s1.1.crossover).unwrap();
        eval.crossover(&mut s1.0, &mut s2.0, idx);
        Ok(())
    }

    fn mutation(&self, mutation: &Mutation, eval: &E, s: &mut State<E>) -> Result<()> {
        match mutation {
            Mutation::Fixed(rates) => {
                s.1.mutation = rates.clone();
            }
            Mutation::Adaptive => {
                // Apply every mutation with the given rate.
                // c' = c * e^(learning rate * N(0, 1))
                let lrate = 1.0 / (self.size() as f64).sqrt();
                mutate_rate(&mut s.1.mutation, 1.0, |v| mutate_lognorm(v, lrate).clamp(0.0, 1.0));
            }
        };
        Self::check_weights(&s.1.mutation, E::NUM_MUTATION)?;
        for (idx, &rate) in s.1.mutation.iter().enumerate() {
            eval.mutate(&mut s.0, rate, idx);
        }
        Ok(())
    }


    pub fn next_gen(&self, cfg: &Cfg, eval: &E) -> Result<UnevaluatedGen<E>> {
        // Pick survivors:
        let mut new_states = self.survivors(cfg.survival);
        let remaining = cfg.pop_size - new_states.len();
        // Reproduce:
        let reproduced = (0..remaining)
            .into_par_iter()
            .map(|_| {
                let [mut s1, mut s2] = self.selection(cfg.selection);
                self.crossover(&cfg.crossover, eval, &mut s1, &mut s2).unwrap();
                self.mutation(&cfg.mutation, eval, &mut s1).unwrap();
                self.mutation(&cfg.mutation, eval, &mut s2).unwrap();
                vec![s1, s2].into_iter()
            })
            .flatten_iter()
            .collect::<Vec<_>>();
        new_states.extend(reproduced.into_iter().take(remaining));
        Ok(UnevaluatedGen::new(new_states))
    }
}
