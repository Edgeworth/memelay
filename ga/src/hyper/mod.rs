use std::marker::PhantomData;
use std::time::{Duration, Instant};

use num_traits::abs;
use rand::Rng;
use rand_distr::{Distribution, Standard};

use crate::cfg::{Cfg, Crossover, Mutation, Niching, Species, Survival};
use crate::gen::unevaluated::UnevaluatedGen;
use crate::ops::distance::{dist2, dist_abs};
use crate::ops::util::rand_vec;
use crate::runner::{Runner, RunnerFn};
use crate::Evaluator;

// TODO: GA for optimising hyper-params of a GA, i.e:
// Cfg settings + which crossover and mutation operators etc to use
// Also try with average of all example problems.

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct State {
    cfg: Cfg,
    crossover: Vec<f64>, // Weights for fixed crossover.
    mutation: Vec<f64>,  // Weights for fixed mutation.
}

impl Distribution<State> for Standard {
    fn sample<R: Rng + ?Sized>(&self, r: &mut R) -> State {
        State { cfg: Cfg::new(100), crossover: (), mutation: () }
    }
}

#[derive(Debug, Clone)]
pub struct HyperGA<F: RunnerFn<E>, E: Evaluator> {
    runner: F,
    _e: PhantomData<E>,
}

impl<F: RunnerFn<E>, E: Evaluator> HyperGA<F, E> {
    pub fn new(runner: F) -> Self {
        Self { runner, _e: PhantomData }
    }
}

impl<F: RunnerFn<E>, E: Evaluator> Evaluator for HyperGA<F, E> {
    type Genome = State;

    fn crossover(&self, s1: &mut State, s2: &mut State, idx: usize) {
        todo!()
    }

    fn mutate(&self, s: &mut State, rate: f64, idx: usize) {
        todo!()
    }

    fn fitness(&self, s: &State) -> f64 {
        const SAMPLES: usize = 30;
        const SAMPLE_DUR: Duration = Duration::from_millis(100);
        let mut score = 0.0;
        for _ in 0..SAMPLES {
            let mut runner = (self.runner)(s.cfg.clone());
            let st = Instant::now();
            while (Instant::now() - st) < SAMPLE_DUR {
                runner.run_iter(false).unwrap();
            }
            let r = runner.run_iter(true).unwrap().stats.unwrap();
            score += r.best_fitness;
            // TODO: Need multi-objective GA here.

            // g.add(&format!("{}:{}:best fitness", name, cfg_name), run_id, r.best_fitness);
            // g.add(&format!("{}:{}:mean fitness", name, cfg_name), run_id, r.mean_fitness);
            // g.add(&format!("{}:{}:dupes", name, cfg_name), run_id, r.num_dup as f64);
            // g.add(&format!("{}:{}:mean dist", name, cfg_name), run_id, r.mean_distance);
            // g.add(&format!("{}:{}:species", name, cfg_name), run_id, r.num_species as f64);
        }
        score / SAMPLES as f64
    }

    fn distance(&self, s1: &State, s2: &State) -> f64 {
        let mut dist = dist_abs(s1.cfg.pop_size, s2.cfg.pop_size) as f64;

        let s1_cross = &if let Crossover::Fixed(v) = s1.cfg.crossover { v } else { s1.crossover };
        let s2_cross = &if let Crossover::Fixed(v) = s2.cfg.crossover { v } else { s2.crossover };
        dist += dist2(s1_cross, s2_cross);

        let s1_mutation = &if let Mutation::Fixed(v) = s1.cfg.mutation { v } else { s1.mutation };
        let s2_mutation = &if let Mutation::Fixed(v) = s2.cfg.mutation { v } else { s2.mutation };
        dist += dist2(s1_mutation, s2_mutation);

        dist
    }
}

pub fn hyper_runner<F: RunnerFn<E>, E: Evaluator>(runner: F) -> Runner<HyperGA<F, E>> {
    let mut r = rand::thread_rng();
    let cfg = Cfg::new(100)
        .with_mutation(Mutation::Adaptive(2))
        .with_crossover(Crossover::Adaptive(2))
        .with_survival(Survival::SpeciesTopProportion(0.1))
        .with_species(Species::TargetNumber(10))
        .with_niching(Niching::SharedFitness);
    let initial = rand_vec(cfg.pop_size, || r.gen()); // TODO fix
    let gen = UnevaluatedGen::initial(initial);
    Runner::new(HyperGA::new(runner), cfg, gen)
}
