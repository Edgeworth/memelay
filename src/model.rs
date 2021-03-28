use std::collections::HashMap;

use crate::layout::Layout;
use crate::types::Kc;

const SWITCH_HAND: f64 = -1.0; // Alternating hands is easy.
const SAME_KEY: f64 = 0.0; // Same key is neither easy nor hard.

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Model {
    pub layout: String,
    pub keys: Vec<Kc>,
    pub fixed: Vec<Kc>,
    pub unigram_cost: Vec<f64>,
    pub bigram_cost: [[[f64; 3]; 4]; 4],
    pub row: Vec<i32>,
    pub hand: Vec<i32>,
    pub finger: Vec<i32>,
}

impl Model {
    pub fn unigram_cost(&self, l: &Layout, unigrams: &HashMap<Kc, f64>) -> f64 {
        let mut cost = 0.0;
        for (&kc, &prop) in unigrams.iter() {
            // Finger penalties - penalise for not being able to type characters.
            let percost = if let Some(curi) = l.keys.iter().position(|&v| v == kc) {
                self.unigram_cost[curi]
            } else {
                100.0
            };
            cost += percost * prop;
        }
        cost
    }

    pub fn bigram_cost(&self, l: &Layout, bigrams: &HashMap<(Kc, Kc), f64>) -> f64 {
        let mut cost = 0.0;
        for (&(kc1, kc2), &prop) in bigrams.iter() {
            // Model adapted from https://colemakmods.github.io/mod-dh/compare.html
            let previ = l.keys.iter().position(|&v| v == kc1);
            let curi = l.keys.iter().position(|&v| v == kc2);
            if previ.is_none() || curi.is_none() {
                continue;
            }
            let previ = previ.unwrap();
            let curi = curi.unwrap();
            let pfing = self.finger[previ] as usize;
            let cfing = self.finger[curi] as usize;
            let same_hand = self.hand[previ] == self.hand[curi];
            let jump_len = (self.row[curi] - self.row[previ]).abs() as usize;

            // Special case: same key incurs zero cost for bigrams.
            // Index finger can be used twice on the same row with different keys.
            let percost = if same_hand {
                if kc1 != kc2 { self.bigram_cost[pfing][cfing][jump_len] } else { SAME_KEY }
            } else {
                SWITCH_HAND
            };
            cost += percost * prop;
        }
        cost
    }

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
        s.truncate(s.trim_end().len());
        s
    }

    pub fn without_fixed(&self, inp: &[Kc]) -> Vec<Kc> {
        assert_eq!(inp.len(), self.keys.len(), "must have same size when removing fixed");
        let mut out: Vec<Kc> = Vec::with_capacity(self.keys.len());
        for i in 0..inp.len() {
            if self.fixed[i] == Kc::None {
                out.push(inp[i]);
            }
        }
        out
    }

    pub fn with_fixed(&self, inp: &[Kc]) -> Vec<Kc> {
        let mut out: Vec<Kc> = Vec::with_capacity(self.keys.len());
        let mut idx = 0;
        for i in 0..self.keys.len() {
            if self.fixed[i] == Kc::None {
                out.push(inp[idx]);
                idx += 1;
            } else {
                out.push(self.fixed[i])
            }
        }
        assert_eq!(idx, inp.len(), "left over keys");
        out
    }
}
