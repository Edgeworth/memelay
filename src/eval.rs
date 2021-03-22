use crate::ingest::{load_histograms, load_params};
use crate::layout::{Layout, COLEMAK_DHM_KEYS};
use crate::types::Kc;
use crate::Args;
use eyre::Result;
use memega::ops::crossover::{crossover_cycle, crossover_order, crossover_pmx};
use memega::ops::distance::kendall_tau;
use memega::ops::mutation::{
    mutate_gen, mutate_insert, mutate_inversion, mutate_rate, mutate_scramble, mutate_swap,
};
use memega::Evaluator;
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Params {
    pub layout: String,
    pub cost: Vec<f64>,
    pub row: Vec<i32>,
    pub hand: Vec<i32>,
    pub finger: Vec<i32>,
}

impl Params {
    pub fn format(&self, l: &Layout) -> String {
        let mut s = String::new();
        let mut idx = 0;
        for c in self.layout.chars() {
            if c == 'X' {
                s += &format!("{}", l.keys[idx]);
                idx += 1;
            } else {
                s.push(c);
            }
        }
        s.push('\n');
        s
    }
}

#[derive(Debug, Clone)]
pub struct Histograms {
    pub unigrams: HashMap<Kc, f64>,
    pub bigrams: HashMap<(Kc, Kc), f64>,
}

#[derive(Debug, Clone)]
pub struct LayoutEval {
    pub params: Params,
    pub match_keys: Vec<Kc>,
    pub hist: Histograms,
}

impl LayoutEval {
    pub fn from_args(args: &Args) -> Result<Self> {
        let params = load_params(&args.params_path)?;
        let hist = load_histograms(&args.unigrams_path, &args.bigrams_path)?;
        Ok(Self { params, hist, match_keys: COLEMAK_DHM_KEYS.to_vec() })
    }
}

impl Evaluator for LayoutEval {
    type Genome = Layout;
    const NUM_CROSSOVER: usize = 4;
    const NUM_MUTATION: usize = 5;

    fn crossover(&self, s1: &mut Layout, s2: &mut Layout, idx: usize) {
        match idx {
            0 => {} // Do nothing.
            1 => {
                crossover_pmx(&mut s1.keys, &mut s2.keys);
            }
            2 => {
                crossover_order(&mut s1.keys, &mut s2.keys);
            }
            3 => {
                crossover_cycle(&mut s1.keys, &mut s2.keys);
            }
            _ => panic!("unknown crossover strategy"),
        };
    }

    fn mutate(&self, s: &mut Layout, rate: f64, idx: usize) {
        let mut r = rand::thread_rng();
        let mutate = r.gen::<f64>() < rate;
        match idx {
            0 => {
                // Mutate random available key.
                mutate_rate(&mut s.keys, rate, |_| mutate_gen());
            }
            1 => {
                if mutate {
                    mutate_swap(&mut s.keys);
                }
            }
            2 => {
                if mutate {
                    mutate_insert(&mut s.keys);
                }
            }
            3 => {
                if mutate {
                    mutate_scramble(&mut s.keys);
                }
            }
            4 => {
                if mutate {
                    mutate_inversion(&mut s.keys);
                }
            }
            _ => panic!("unknown mutation strategy"),
        }
    }

    fn fitness(&self, s: &Layout) -> f64 {
        // Indexed by: first finger, second finger, row jump amount
        // Values adapted from https://github.com/bclnr/kb-layout-evaluation
        const BIGRAM_MAP: [[[f64; 3]; 4]; 4] = [
            [
                // First finger: index
                [2.5, 3.0, 4.0], // Index - same row val only used for different key locations
                [0.5, 1.0, 2.0], // Middle - outward roll
                [0.5, 0.8, 1.5], // Ring - outward roll
                [0.5, 0.7, 1.1], // Pinkie - outward roll
            ],
            [
                // First finger: middle
                [-1.5, -0.5, 1.5], // Index - inward roll
                [0.0, 3.5, 4.5],   // Middle - same row val only used for different key locations
                [0.5, 1.0, 2.0],   // Ring - outward roll
                [0.5, 0.8, 1.5],   // Pinkie - outward roll
            ],
            [
                // First finger: ring
                [-1.5, -0.5, 1.5], // Index - inward roll
                [-2.0, -0.5, 1.2], // Middle - inward roll
                [0.0, 3.5, 4.5],   // Ring - same row val only used for different key locations
                [0.0, 3.5, 4.5],   // Pinkie - outward roll
            ],
            [
                // First finger: pinkie
                [-1.0, 0.0, 1.0], // Index - inward roll
                [-1.0, 0.0, 1.5], // Middle - inward roll
                [-1.0, 0.0, 1.5], // Ring - inward roll
                [3.0, 4.0, 5.5],  // Pinkie - same row val only used for different key locations
            ],
        ];
        const SWITCH_HAND: f64 = -4.0; // Alternating hands is very easy.
        const SAME_KEY: f64 = 0.0; // Same key is neither easy nor hard.
        // Treat bigram data as less important than unigram.
        // Equally, trigram would be less important than bigram.
        const BIGRAM_MULT: f64 = 0.5;

        let mut cost = 0.0;
        // Check unigrams:
        for (&kc, &prop) in self.hist.unigrams.iter() {
            // Finger penalties - penalise for not being able to type characters.
            let percost = if let Some(curi) = s.keys.iter().position(|&v| v == kc) {
                self.params.cost[curi]
            } else {
                100.0
            };
            cost += percost * prop;
        }

        // Check bi-grams
        for (&(kc1, kc2), &prop) in self.hist.bigrams.iter() {
            // Model adapted from https://colemakmods.github.io/mod-dh/compare.html
            let previ = s.keys.iter().position(|&v| v == kc1);
            let curi = s.keys.iter().position(|&v| v == kc2);
            if previ.is_none() || curi.is_none() {
                continue;
            }
            let previ = previ.unwrap();
            let curi = curi.unwrap();
            let pfing = self.params.finger[previ] as usize;
            let cfing = self.params.finger[curi] as usize;
            let same_hand = self.params.hand[previ] == self.params.hand[curi];
            let jump_len = (self.params.row[curi] - self.params.row[previ]).abs() as usize;

            // Special case: same key incurs zero cost for bigrams.
            // Index finger can be used twice on the same row with different keys.
            let percost = if same_hand {
                if kc1 != kc2 { BIGRAM_MAP[pfing][cfing][jump_len] } else { SAME_KEY }
            } else {
                SWITCH_HAND
            };
            cost += percost * BIGRAM_MULT * prop;
        }

        // Tie-breaking: similarity to given existing layout:
        cost += kendall_tau(&s.keys, &self.match_keys).unwrap() as f64 / 10000.0;

        // 1.0 / (cost + 1.0)
        (-cost).exp()
    }

    fn distance(&self, s1: &Layout, s2: &Layout) -> f64 {
        kendall_tau(&s1.keys, &s2.keys).unwrap() as f64
    }
}
