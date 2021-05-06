use eyre::Result;
use memega::eval::Evaluator;
use memega::ops::crossover::{crossover_cycle, crossover_order, crossover_pmx};
use memega::ops::distance::count_different;
use memega::ops::mutation::{mutate_insert, mutate_inversion, mutate_scramble, mutate_swap};
use rand::Rng;

use crate::ingest::{load_histograms, load_model};
use crate::model::{Model, PENALTY};
use crate::types::{Kc, COLEMAK_DHM};
use crate::Args;

#[derive(Debug, Clone)]
pub struct Histograms {
    pub unigrams: Vec<(Kc, f64)>,
    pub bigrams: Vec<((Kc, Kc), f64)>,
    pub trigrams: Vec<((Kc, Kc, Kc), f64)>,
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
        let hist = load_histograms(&args.unigrams_path, &args.bigrams_path, &args.trigrams_path)?;
        Ok(Self { model, hist, match_keys: COLEMAK_DHM.to_vec() })
    }
}

impl Evaluator for LayoutEval {
    type Genome = Vec<Kc>;
    const NUM_CROSSOVER: usize = 4;
    const NUM_MUTATION: usize = 4;

    fn crossover(&self, s1: &mut Vec<Kc>, s2: &mut Vec<Kc>, idx: usize) {
        // Crossover without touching fixed keys.
        let mut unfixed1 = self.model.without_fixed(&s1);
        let mut unfixed2 = self.model.without_fixed(&s2);
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
        *s1 = self.model.with_fixed(&unfixed1);
        *s2 = self.model.with_fixed(&unfixed2);
    }

    fn mutate(&self, s: &mut Vec<Kc>, rate: f64, idx: usize) {
        let mut r = rand::thread_rng();
        let mutate = r.gen::<f64>() < rate;
        // Mutate without touching fixed keys.
        let mut unfixed = self.model.without_fixed(&s);
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
        *s = self.model.with_fixed(&unfixed);
    }

    fn fitness(&self, s: &Vec<Kc>) -> f64 {
        let mut cost = 0.0;

        cost += self.model.unigram_cost(s, &self.hist.unigrams);
        cost += self.model.bigram_cost(s, &self.hist.bigrams);
        cost += self.model.trigram_cost(s, &self.hist.trigrams);

        let comma = s.iter().position(|&v| v == Kc::Comma);
        let dot = s.iter().position(|&v| v == Kc::Dot);
        if dot.is_some() && comma.is_some() && comma.unwrap() + 1 != dot.unwrap() {
            cost += PENALTY; // Keep , and . next to eachother.
        }

        // Check fixed keys
        for (i, &kc) in self.model.fixed.iter().enumerate() {
            if kc != Kc::None && kc != s[i] {
                cost += PENALTY;
            }
        }

        // Tie-breaking: similarity to given existing layout:
        cost += count_different(&s, &self.match_keys) as f64 / 100000.0;

        // 1.0 / (cost + 1.0)
        (-cost).exp()
    }

    fn distance(&self, s1: &Vec<Kc>, s2: &Vec<Kc>) -> f64 {
        let mut d = 0.0;
        for i in 0..s1.len() {
            d += (i8::from(s1[i]) - i8::from(s2[i])).abs() as f64;
        }
        d
    }
}
