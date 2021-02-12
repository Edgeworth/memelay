use crate::cfg::{Cfg, Crossover, Mutation, Niching, Species, Survival};
use crate::gen::unevaluated::UnevaluatedGen;
use crate::ops::distance::{dist2, dist_abs};
use crate::ops::util::rand_vec;
use crate::runner::{Runner, RunnerFn};
use crate::{Evaluator, Genome};
use rand::Rng;
use rand_distr::{Distribution, Standard};
use std::marker::PhantomData;
use std::time::{Duration, Instant};

// TODO: GA for optimising hyper-params of a GA, i.e:
// Cfg settings + which crossover and mutation operators etc to use
// Also try with average of all example problems.

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct State<E: Evaluator> {
    cfg: Cfg,
    crossover: Vec<f64>, // Weights for fixed crossover.
    mutation: Vec<f64>,  // Weights for fixed mutation.
    _e: PhantomData<E>,
}

impl<E: Evaluator> Distribution<State<E>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, r: &mut R) -> State<E> {
        let crossover = rand_vec(E::NUM_CROSSOVER, || r.gen());
        let mutation = rand_vec(E::NUM_MUTATION, || r.gen());
        // TODO: randomise cfg too.
        State { cfg: Cfg::new(100), crossover, mutation, _e: PhantomData }
    }
}

#[derive(Debug, Clone)]
pub struct HyperAlg<F: RunnerFn<E>, E: Evaluator + Genome> {
    runner: F,
    _e: PhantomData<E>,
}

impl<F: RunnerFn<E>, E: Evaluator + Genome> HyperAlg<F, E> {
    pub fn new(runner: F) -> Self {
        Self { runner, _e: PhantomData }
    }
}

impl<F: RunnerFn<E>, E: Evaluator + Genome> Evaluator for HyperAlg<F, E> {
    type Genome = State<E>;
    const NUM_CROSSOVER: usize = 2;
    const NUM_MUTATION: usize = 7;

    fn crossover(&self, s1: &mut State<E>, s2: &mut State<E>, idx: usize) {
        match idx {
            0 => {}
            1 => {}
            _ => panic!("bug"),
        }
    }

    fn mutate(&self, s: &mut State<E>, rate: f64, idx: usize) {
        let mut r = rand::thread_rng();
        match idx {
            0 => {}
            1 => {
                // Mutate crossover
                s.cfg.crossover = r.gen();
            }
            2 => {
                // Mutate mutation
                s.cfg.mutation = r.gen();
            }
            3 => {
                // Mutate survival
                s.cfg.survival = r.gen();
            }
            4 => {
                // Mutate selection
                s.cfg.selection = r.gen();
            }
            5 => {
                // Mutate niching
                s.cfg.niching = r.gen();
            }
            6 => {
                // Mutate species
                s.cfg.species = r.gen();
            }
            _ => panic!("bug"),
        }
    }

    fn fitness(&self, s: &State<E>) -> f64 {
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

    fn distance(&self, s1: &State<E>, s2: &State<E>) -> f64 {
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

pub fn hyper_runner<F: RunnerFn<E>, E: Evaluator + Genome>(runner: F) -> Runner<HyperAlg<F, E>> {
    let mut r = rand::thread_rng();
    let cfg = Cfg::new(100)
        .with_mutation(Mutation::Adaptive)
        .with_crossover(Crossover::Adaptive)
        .with_survival(Survival::SpeciesTopProportion(0.1))
        .with_species(Species::TargetNumber(10))
        .with_niching(Niching::SharedFitness);
    let initial = rand_vec(cfg.pop_size, || r.gen()); // TODO fix
    let gen = UnevaluatedGen::initial(initial, &cfg);
    Runner::new(HyperAlg::new(runner), cfg, gen)
}
