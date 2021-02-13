use crate::cfg::{Cfg, Crossover, Mutation, Niching, Species, Survival};
use crate::gen::unevaluated::UnevaluatedGen;
use crate::ops::distance::{dist2, dist_abs};
use crate::ops::mutation::{mutate_creep, mutate_normal, mutate_rate};
use crate::ops::util::rand_vec;
use crate::runner::{Runner, RunnerFn};
use crate::Evaluator;
use rand::Rng;
use std::marker::PhantomData;
use std::mem::swap;
use std::time::{Duration, Instant};

const MAX_POP: usize = 1000;

// Also try with average of all example problems.

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct State {
    cfg: Cfg,
    crossover: Vec<f64>, // Weights for fixed crossover.
    mutation: Vec<f64>,  // Weights for fixed mutation.
}

fn rand_state<E: Evaluator>() -> State {
    let mut r = rand::thread_rng();
    let crossover = rand_vec(E::NUM_CROSSOVER, || r.gen());
    let mutation = rand_vec(E::NUM_MUTATION, || r.gen());
    let mut cfg = Cfg::new(r.gen_range(5..MAX_POP));
    cfg.survival = r.gen();
    cfg.selection = r.gen();
    cfg.niching = r.gen();
    cfg.species = r.gen();
    State { cfg, crossover, mutation }
}

#[derive(Debug, Clone)]
pub struct HyperAlg<F: RunnerFn<E>, E: Evaluator> {
    runner: F,
    _e: PhantomData<E>,
}

impl<F: RunnerFn<E>, E: Evaluator> HyperAlg<F, E> {
    pub fn new(runner: F) -> Self {
        Self { runner, _e: PhantomData }
    }
}

impl<F: RunnerFn<E>, E: Evaluator> Evaluator for HyperAlg<F, E> {
    type Genome = State;
    const NUM_CROSSOVER: usize = 2;
    const NUM_MUTATION: usize = 10;

    fn crossover(&self, s1: &mut State, s2: &mut State, idx: usize) {
        let mut r = rand::thread_rng();
        match idx {
            0 => {}
            1 => {
                // Uniform crossover-like operation:
                if r.gen::<bool>() {
                    swap(&mut s1.cfg.pop_size, &mut s2.cfg.pop_size);
                }
                if r.gen::<bool>() {
                    swap(&mut s1.cfg.crossover, &mut s2.cfg.crossover);
                }
                if r.gen::<bool>() {
                    swap(&mut s1.cfg.mutation, &mut s2.cfg.mutation);
                }
                if r.gen::<bool>() {
                    swap(&mut s1.cfg.survival, &mut s2.cfg.survival);
                }
                if r.gen::<bool>() {
                    swap(&mut s1.cfg.selection, &mut s2.cfg.selection);
                }
                if r.gen::<bool>() {
                    swap(&mut s1.cfg.niching, &mut s2.cfg.niching);
                }
                if r.gen::<bool>() {
                    swap(&mut s1.cfg.species, &mut s2.cfg.species);
                }
                if r.gen::<bool>() {
                    swap(&mut s1.crossover, &mut s2.crossover);
                }
                if r.gen::<bool>() {
                    swap(&mut s1.mutation, &mut s2.mutation);
                }
            }
            _ => panic!("bug"),
        }
    }

    fn mutate(&self, s: &mut State, rate: f64, idx: usize) {
        let mut r = rand::thread_rng();
        match idx {
            0 => {
                // Mutate crossover - change type
                if r.gen_bool(rate) {
                    match &s.cfg.crossover {
                        Crossover::Fixed(v) => {
                            s.crossover = v.clone();
                            s.cfg.crossover = Crossover::Adaptive;
                        }
                        Crossover::Adaptive => {
                            s.cfg.crossover = Crossover::Fixed(s.crossover.clone());
                        }
                    }
                }
            }
            1 => {
                // Mutate crossover - modify weights
                match &mut s.cfg.crossover {
                    Crossover::Fixed(v) => {
                        mutate_rate(v, 1.0, |v| mutate_normal(v, rate).max(0.0));
                    }
                    Crossover::Adaptive => {
                        mutate_rate(&mut s.crossover, 1.0, |v| mutate_normal(v, rate).max(0.0));
                    }
                }
            }
            2 => {
                // Mutate mutation - change type
                if r.gen_bool(rate) {
                    match &s.cfg.mutation {
                        Mutation::Fixed(v) => {
                            s.mutation = v.clone();
                            s.cfg.mutation = Mutation::Adaptive;
                        }
                        Mutation::Adaptive => {
                            s.cfg.mutation = Mutation::Fixed(s.mutation.clone());
                        }
                    }
                }
            }
            3 => {
                // Mutate mutation - modify weights
                match &mut s.cfg.mutation {
                    Mutation::Fixed(v) => {
                        mutate_rate(v, 1.0, |v| mutate_normal(v, rate).max(0.0));
                    }
                    Mutation::Adaptive => {
                        mutate_rate(&mut s.mutation, 1.0, |v| mutate_normal(v, rate).max(0.0));
                    }
                }
            }
            4 => {
                if r.gen_bool(rate) {
                    s.cfg.survival = r.gen()
                }
            }
            5 => {
                if r.gen_bool(rate) {
                    s.cfg.selection = r.gen()
                }
            }
            6 => {
                if r.gen_bool(rate) {
                    s.cfg.niching = r.gen()
                }
            }
            7 => {
                if r.gen_bool(rate) {
                    s.cfg.species = r.gen()
                }
            }
            8 => {
                if r.gen_bool(rate) {
                    s.cfg.pop_size = mutate_creep(s.cfg.pop_size, 10).clamp(2, MAX_POP);
                }
            }
            9 => {
                if r.gen_bool(rate) {
                    s.cfg.pop_size = r.gen_range(2..MAX_POP)
                }
            }
            _ => panic!("bug"),
        }
    }

    fn fitness(&self, s: &State) -> f64 {
        const SAMPLES: usize = 100;
        const SAMPLE_DUR: Duration = Duration::from_millis(10);
        let mut score = 0.0;
        for _ in 0..SAMPLES {
            let mut runner = (self.runner)(s.cfg.clone());
            let st = Instant::now();
            // Get the last run that ran in time.
            let mut r1 = None;
            let mut r2 = None;
            while (Instant::now() - st) < SAMPLE_DUR {
                swap(&mut r1, &mut r2);
                r2 = Some(runner.run_iter(true).unwrap().stats.unwrap());
            }
            if let Some(r) = r2 {
                score += r.mean_fitness;
            }
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

        let s1_cross = if let Crossover::Fixed(v) = &s1.cfg.crossover { v } else { &s1.crossover };
        let s2_cross = if let Crossover::Fixed(v) = &s2.cfg.crossover { v } else { &s2.crossover };
        dist += dist2(s1_cross, s2_cross);

        let s1_mutation = if let Mutation::Fixed(v) = &s1.cfg.mutation { v } else { &s1.mutation };
        let s2_mutation = if let Mutation::Fixed(v) = &s2.cfg.mutation { v } else { &s2.mutation };
        dist += dist2(s1_mutation, s2_mutation);

        dist
    }
}

pub fn hyper_runner<F: RunnerFn<E>, E: Evaluator>(runner: F) -> Runner<HyperAlg<F, E>> {
    let cfg = Cfg::new(100)
        .with_mutation(Mutation::Adaptive)
        .with_crossover(Crossover::Adaptive)
        .with_survival(Survival::SpeciesTopProportion(0.1))
        .with_species(Species::TargetNumber(10))
        .with_niching(Niching::None);
    let initial = rand_vec(cfg.pop_size, rand_state::<E>); // TODO fix
    let gen = UnevaluatedGen::initial(initial, &cfg);
    Runner::new(HyperAlg::new(runner), cfg, gen)
}
