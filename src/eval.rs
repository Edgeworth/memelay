use crate::ingest::{load_histograms, load_model};
use crate::layout::{Layout, COLEMAK_DHM_KEYS};
use crate::model::Model;
use crate::types::Kc;
use crate::Args;
use eyre::Result;
use memega::eval::Evaluator;
use memega::ops::crossover::{crossover_cycle, crossover_order, crossover_pmx};
use memega::ops::distance::count_different;
use memega::ops::mutation::{mutate_insert, mutate_inversion, mutate_scramble, mutate_swap};
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Histograms {
    pub unigrams: HashMap<Kc, f64>,
    pub bigrams: HashMap<(Kc, Kc), f64>,
}

#[derive(Debug, Clone)]
pub struct LayoutEval {
    pub model: Model,
    pub match_keys: Vec<Kc>,
    pub hist: Histograms,
}

impl LayoutEval {
    pub fn from_args(args: &Args) -> Result<Self> {
        let model = load_model(&args.model_path)?;
        let hist = load_histograms(&args.unigrams_path, &args.bigrams_path)?;
        Ok(Self { model, hist, match_keys: COLEMAK_DHM_KEYS.to_vec() })
    }
}

impl Evaluator for LayoutEval {
    type Genome = Layout;
    const NUM_CROSSOVER: usize = 4;
    const NUM_MUTATION: usize = 4;

    fn crossover(&self, s1: &mut Layout, s2: &mut Layout, idx: usize) {
        // Crossover without touching fixed keys.
        let mut unfixed1 = self.model.without_fixed(&s1.keys);
        let mut unfixed2 = self.model.without_fixed(&s2.keys);
        match idx {
            0 => {} // Do nothing.
            1 => {
                crossover_pmx(&mut unfixed1, &mut unfixed2);
            }
            2 => {
                crossover_order(&mut unfixed1, &mut unfixed2);
            }
            3 => {
                crossover_cycle(&mut unfixed1, &mut unfixed2);
            }
            _ => panic!("unknown crossover strategy"),
        };
        s1.keys = self.model.with_fixed(&unfixed1);
        s2.keys = self.model.with_fixed(&unfixed2);
    }

    fn mutate(&self, s: &mut Layout, rate: f64, idx: usize) {
        let mut r = rand::thread_rng();
        let mutate = r.gen::<f64>() < rate;
        // Mutate without touching fixed keys.
        let mut unfixed = self.model.without_fixed(&s.keys);
        match idx {
            0 => {
                if mutate {
                    mutate_swap(&mut unfixed);
                }
            }
            1 => {
                if mutate {
                    mutate_insert(&mut unfixed);
                }
            }
            2 => {
                if mutate {
                    mutate_scramble(&mut unfixed);
                }
            }
            3 => {
                if mutate {
                    mutate_inversion(&mut unfixed);
                }
            }
            _ => panic!("unknown mutation strategy"),
        }
        s.keys = self.model.with_fixed(&unfixed);
    }

    fn fitness(&self, s: &Layout) -> f64 {
        const FIXED_COST: f64 = 10.0; // Penalty for missing a fixed key.

        let mut cost = 0.0;

        cost += self.model.unigram_cost(s, &self.hist.unigrams);
        cost += self.model.bigram_cost(s, &self.hist.bigrams);

        let comma = s.keys.iter().position(|&v| v == Kc::Comma);
        let dot = s.keys.iter().position(|&v| v == Kc::Dot);
        if dot.is_some() && comma.is_some() && comma.unwrap() + 1 != dot.unwrap() {
            cost += FIXED_COST; // Keep , and . next to eachother.
        }

        // Check fixed keys
        for (i, &kc) in self.model.fixed.iter().enumerate() {
            if kc != Kc::None && kc != s.keys[i] {
                cost += FIXED_COST;
            }
        }

        // Tie-breaking: similarity to given existing layout:
        cost += count_different(&s.keys, &self.match_keys) as f64 / 100000.0;

        // 1.0 / (cost + 1.0)
        (-cost).exp()
    }

    fn distance(&self, s1: &Layout, s2: &Layout) -> f64 {
        let mut d = 0.0;
        for i in 0..s1.keys.len() {
            d += (i8::from(s1.keys[i]) - i8::from(s2.keys[i])).abs() as f64;
        }
        d
    }
}
