use crate::types::Kc;
use std::collections::HashMap;

const SWITCH_HAND: f64 = -0.5; // Alternating hands is easy.
const SAME_KEY: f64 = 0.0; // Same key is neither easy nor hard.
pub const PENALTY: f64 = 100.0;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Model {
    pub layout: String,    // Format string for printing keyboard layouts
    pub universe: Vec<Kc>, // What keys we can use
    pub fixed: Vec<Kc>,    // Positions of keys that should be fixed in place.
    pub unigram_cost: Vec<f64>,
    pub bigram_cost: [[[f64; 5]; 4]; 4],
    pub row: Vec<i32>,
    pub hand: Vec<i32>,
    pub finger: Vec<i32>,
}

impl Model {
    pub fn unigram_cost(&self, l: &[Kc], unigrams: &HashMap<Kc, f64>) -> f64 {
        let mut cost = 0.0;
        for (&kc, &prop) in unigrams.iter() {
            // Finger penalties - penalise for not being able to type characters.
            let percost = if let Some(curi) = l.iter().position(|&v| v == kc) {
                self.unigram_cost[curi]
            } else {
                100.0
            };
            cost += percost * prop;
        }
        cost
    }

    pub fn bigram_cost(&self, l: &[Kc], bigrams: &HashMap<(Kc, Kc), f64>) -> f64 {
        let mut cost = 0.0;
        for (&(kc1, kc2), &prop) in bigrams.iter() {
            // Model adapted from https://colemakmods.github.io/mod-dh/compare.html
            let previ = l.iter().position(|&v| v == kc1);
            let curi = l.iter().position(|&v| v == kc2);
            if previ.is_none() || curi.is_none() {
                continue;
            }
            let previ = previ.unwrap();
            let curi = curi.unwrap();
            let pfing = self.finger[previ] as usize;
            let cfing = self.finger[curi] as usize;
            let same_hand = self.hand[previ] == self.hand[curi];
            let jump_len = (self.row[curi] - self.row[previ] + 2) as usize;

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

    pub fn trigram_cost(&self, l: &[Kc], trigrams: &HashMap<(Kc, Kc, Kc), f64>) -> f64 {
        const ALT_ROLL_BONUS: f64 = -1.0;
        let mut cost = 0.0;
        for (&(kc1, kc2, kc3), &prop) in trigrams.iter() {
            // Model adapted from https://colemakmods.github.io/mod-dh/compare.html
            let i1 = l.iter().position(|&v| v == kc1);
            let i2 = l.iter().position(|&v| v == kc2);
            let i3 = l.iter().position(|&v| v == kc3);
            if i1.is_none() || i2.is_none() || i3.is_none() {
                continue;
            }
            // Bonus for rolling inward on one hand then swithing hand.
            let i1 = i1.unwrap();
            let i2 = i2.unwrap();
            let i3 = i3.unwrap();

            let alt = self.hand[i1] == self.hand[i2] && self.hand[i2] != self.hand[i3];
            let rolling = self.finger[i1] > self.finger[i2];
            let percost = if alt && rolling { ALT_ROLL_BONUS } else { 0.0 };
            cost += percost * prop;
        }
        cost
    }

    pub fn format(&self, l: &[Kc]) -> String {
        let mut s = String::new();
        let mut idx = 0;
        for c in self.layout.chars() {
            if c == 'X' {
                s += &format!("{}", l[idx]);
                idx += 1;
            } else {
                s.push(c);
            }
        }
        s.truncate(s.trim_end().len());
        s
    }

    pub fn without_fixed(&self, inp: &[Kc]) -> Vec<Kc> {
        assert_eq!(inp.len(), self.universe.len(), "must have same size when removing fixed");
        let mut out: Vec<Kc> = Vec::with_capacity(self.universe.len());
        for i in 0..inp.len() {
            if self.fixed[i] == Kc::None {
                out.push(inp[i]);
            }
        }
        out
    }

    pub fn with_fixed(&self, inp: &[Kc]) -> Vec<Kc> {
        let mut out: Vec<Kc> = Vec::with_capacity(self.universe.len());
        let mut idx = 0;
        for i in 0..self.universe.len() {
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

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_unigrams() {
        let model = Model { unigram_cost: vec![1.0, 10.0], ..Default::default() };
        let l = &[Kc::A, Kc::B];
        assert_relative_eq!(
            13.0,
            model.unigram_cost(l, &[(Kc::A, 3.0), (Kc::B, 1.0)].iter().cloned().collect())
        );
        assert_relative_eq!(
            PENALTY * 3.0,
            model.unigram_cost(l, &[(Kc::C, 3.0)].iter().cloned().collect())
        );
    }

    #[test]
    fn test_bigrams() {
        let model = Model {
            bigram_cost: [
                [
                    // First finger: index - [down 2, down 1, same row, up 1, up 2]
                    [0.0, 0.0, 2.5, 3.0, 4.0], // Index - same row val only used for different key locations
                    [0.0, 0.0, 0.5, 1.0, 2.0], // Middle - outward roll
                    [0.0, 0.0, 0.5, 0.8, 1.5], // Ring - outward roll
                    [0.0, 0.0, 0.5, 0.7, 1.1], // Pinkie - outward roll
                ],
                [
                    // First finger: middle - [down 2, down 1, same row, up 1, up 2]
                    [0.0, 0.0, -1.5, -0.5, 1.5], // Index - inward roll
                    [0.0, 0.0, 0.0, 3.5, 4.5], // Middle - same row val only used for different key locations
                    [0.0, 0.0, 0.5, 1.0, 2.0], // Ring - outward roll
                    [0.0, 0.0, 0.5, 0.8, 1.5], // Pinkie - outward roll
                ],
                [
                    // First finger: ring - [down 2, down 1, same row, up 1, up 2]
                    [0.0, 0.0, -1.5, -0.5, 1.5], // Index - inward roll
                    [0.0, 0.0, -2.0, -0.5, 1.2], // Middle - inward roll
                    [0.0, 0.0, 0.0, 3.5, 4.5], // Ring - same row val only used for different key locations
                    [0.0, 0.0, 0.0, 3.5, 4.5], // Pinkie - outward roll
                ],
                [
                    // First finger: pinkie - [down 2, down 1, same row, up 1, up 2]
                    [0.0, 0.0, -1.0, 0.0, 1.0], // Index - inward roll
                    [0.0, 0.0, -1.0, 0.0, 1.5], // Middle - inward roll
                    [0.0, 0.0, -1.0, 0.0, 1.5], // Ring - inward roll
                    [0.0, 0.0, 3.0, 4.0, 5.5], // Pinkie - same row val only used for different key locations
                ],
            ],
            ..Default::default()
        };
        let l = &[Kc::A, Kc::B];
    }
}