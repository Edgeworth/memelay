use crate::cfg::{Cfg, Crossover, Mutation, Niching, Selection, Species, Survival};
use crate::examples::ackley::ackley_runner;
use crate::examples::griewank::griewank_runner;
use crate::examples::knapsack::knapsack_runner;
use crate::examples::rastrigin::rastrigin_runner;
use crate::examples::target_string::target_string_runner;
use crate::gen::unevaluated::UnevaluatedGen;
use crate::ops::distance::{dist2, dist_abs};
use crate::ops::mutation::{mutate_creep, mutate_normal, mutate_rate};
use crate::ops::util::rand_vec;
use crate::runner::{Runner, RunnerFn, Stats};
use crate::Evaluator;
use rand::Rng;
use std::mem::swap;
use std::time::{Duration, Instant};

const MAX_POP: usize = 100;

// Also try with average of all example problems.

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct State {
    cfg: Cfg,
    crossover: Vec<f64>, // Weights for fixed crossover.
    mutation: Vec<f64>,  // Weights for fixed mutation.
}

trait StatFn = Fn(Cfg) -> Option<Stats> + Send + Sync;

pub struct HyperAlg {
    stat_fns: Vec<Box<dyn StatFn>>,
}

impl HyperAlg {
    pub fn new(stat_fns: Vec<Box<dyn StatFn>>) -> Self {
        Self { stat_fns }
    }
}

impl Evaluator for HyperAlg {
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
        const SAMPLES: usize = 10;
        let mut score = 0.0;
        for _ in 0..SAMPLES {
            for f in self.stat_fns.iter() {
                if let Some(r) = f(s.cfg.clone()) {
                    score += r.mean_fitness;
                }
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

pub struct HyperBuilder {
    stat_fns: Vec<Box<dyn StatFn>>,
    num_crossover: usize,
    num_mutation: usize,
    sample_dur: Duration,
}

impl HyperBuilder {
    pub fn new(sample_dur: Duration) -> Self {
        Self { stat_fns: Vec::new(), num_crossover: 0, num_mutation: 0, sample_dur }
    }

    fn rand_state(&self) -> State {
        let mut r = rand::thread_rng();
        let crossover = rand_vec(self.num_crossover, || r.gen());
        let mutation = rand_vec(self.num_mutation, || r.gen());
        let mut cfg = Cfg::new(r.gen_range(5..MAX_POP));
        cfg.survival = r.gen();
        cfg.selection = r.gen();
        cfg.niching = r.gen();
        cfg.species = r.gen();
        State { cfg, crossover, mutation }
    }

    pub fn add<F: RunnerFn<E>, E: Evaluator>(
        &mut self,
        max_fitness: f64,
        f: F,
        _name: &'static str,
    ) {
        self.num_crossover = self.num_crossover.max(E::NUM_CROSSOVER);
        self.num_mutation = self.num_mutation.max(E::NUM_MUTATION);
        let sample_dur = self.sample_dur;
        self.stat_fns.push(Box::new(move |cfg| {
            let mut runner = f(cfg);
            let st = Instant::now();
            let mut r1 = None;
            let mut r2 = None;
            let mut count = 0;
            while (Instant::now() - st) < sample_dur {
                swap(&mut r1, &mut r2);
                r2 = Some(runner.run_iter().unwrap());
                count += 1;
            }
            // warn!("{} ran for {} iters", name, count);

            // Get the last run that ran in time.
            if let Some(mut r) = r1 {
                let mut stats = Stats::from_run(&mut r, runner.eval());
                stats.best_fitness /= max_fitness;
                stats.mean_fitness /= max_fitness;
                Some(stats)
            } else {
                None
            }
        }));
    }

    pub fn build(self) -> Runner<HyperAlg> {
        let cfg = Cfg::new(100)
            .with_mutation(Mutation::Adaptive)
            .with_crossover(Crossover::Adaptive)
            .with_survival(Survival::TopProportion(0.25))
            .with_selection(Selection::Sus)
            .with_species(Species::None)
            .with_niching(Niching::None);
        let initial = rand_vec(cfg.pop_size, || self.rand_state());
        let gen = UnevaluatedGen::initial::<HyperAlg>(initial, &cfg);
        Runner::new(HyperAlg::new(self.stat_fns), cfg, gen)
    }
}

pub fn hyper_all() -> Runner<HyperAlg> {
    let mut builder = HyperBuilder::new(Duration::from_millis(100));
    builder.add(1.0, &|cfg| rastrigin_runner(2, cfg), "rastringin");
    builder.add(1.0, &|cfg| griewank_runner(2, cfg), "griewank");
    builder.add(1.0, &|cfg| ackley_runner(2, cfg), "ackley");
    builder.add(1000.0, &knapsack_runner, "knapsack");
    builder.add(12.0, &target_string_runner, "targetstr");
    builder.build()
}
