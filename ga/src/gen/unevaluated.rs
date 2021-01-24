use crate::cfg::{Cfg, Niching, Species, EP};
use crate::gen::evaluated::{EvaluatedGen, Member};
use crate::gen::species::DistCache;
use crate::Evaluator;
use rayon::prelude::*;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct UnevaluatedGen<E: Evaluator> {
    states: Vec<E::State>,
    fitnesses: Vec<f64>,
    species: Vec<i32>,
    num_species: usize,
    species_radius: f64,
    dists: DistCache,
}

impl<E: Evaluator> UnevaluatedGen<E> {
    pub fn from_states(states: Vec<E::State>) -> Self {
        if states.is_empty() {
            panic!("Generation must not be empty");
        }
        Self {
            states,
            fitnesses: Vec::new(),
            species: Vec::new(),
            num_species: 1,
            species_radius: 1.0,
            dists: DistCache::empty(),
        }
    }

    fn ensure_dists(&mut self, cfg: &Cfg, eval: &E) {
        if self.dists.is_empty() {
            self.dists = DistCache::new(cfg, eval, &self.states);
        }
    }

    fn assign_species(&mut self, dist: f64) -> usize {
        let n = self.states.len();
        self.species = vec![-1; n];
        let mut num_species = 0;
        for i in 0..n {
            if self.species[i] == -1 {
                continue;
            }
            self.species[i] = num_species as i32;
            for j in (i + 1)..n {
                if self.dists[(i, j)] <= dist {
                    self.species[j] = num_species as i32;
                }
            }
            num_species += 1;
        }
        num_species
    }


    pub fn evaluate(&mut self, cfg: &Cfg, eval: &E) -> EvaluatedGen<E> {
        // First compute plain fitnesses.
        self.fitnesses = self.states.par_iter_mut().map(|s| eval.fitness(cfg, s)).collect();

        // Speciate if necessary.
        match cfg.species {
            Species::None => {}
            Species::TargetNumber(target) => {
                self.ensure_dists(cfg, eval);
                let mut lo = 0.0;
                let mut hi = self.dists.max();
                // TODO: tests
                while hi - lo > EP {
                    self.species_radius = (lo + hi) / 2.0;
                    self.num_species = self.assign_species(self.species_radius);
                    match self.num_species.cmp(&target) {
                        Ordering::Less => hi = self.species_radius,
                        Ordering::Equal => break,
                        Ordering::Greater => lo = self.species_radius,
                    }
                }
            }
        }

        // Transform fitness if necessary.
        // TODO: tests
        match cfg.niching {
            Niching::None => {}
            Niching::SharedFitness => {
                self.ensure_dists(cfg, eval);

                // Compute fitness as F'(i) = F(i) / sum of 1 - (d(i, j) / species_radius) ^ alpha.
                let alpha = 5.0;
                let n = self.states.len();
                let mut fitnesses = self.fitnesses.clone();
                fitnesses.par_iter_mut().enumerate().for_each(|(i, f)| {
                    let denom = (0..n)
                        .into_par_iter()
                        .map(|j| 1.0 - (self.dists[(i, j)] / self.species_radius).powf(alpha))
                        .sum::<f64>();
                    *f /= denom;
                });
            }
        };
        EvaluatedGen::new(
            (0..self.states.len())
                .map(|i| Member {
                    state: self.states[i].clone(),
                    fitness: self.fitnesses[i],
                    species: *self.species.get(i).unwrap_or(&0),
                })
                .collect(),
        )
    }
}
