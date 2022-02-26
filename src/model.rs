use crate::types::Kc;

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
    #[must_use]
    pub fn unigram_cost(&self, l: &[Kc], unigrams: &[(Kc, f64)]) -> f64 {
        let mut cost = 0.0;
        for &(kc, prop) in unigrams.iter() {
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

    // Assumes key below is on the same hand and finger.
    #[must_use]
    pub fn key_below(&self, v: usize) -> Option<usize> {
        let row = self.row[v] - 1;
        for i in 0..self.row.len() {
            if row == self.row[i]
                && self.hand[i] == self.hand[v]
                && self.finger[i] == self.finger[v]
            {
                return Some(i);
            }
        }
        None
    }

    #[must_use]
    pub fn bigram_cost(&self, l: &[Kc], bigrams: &[((Kc, Kc), f64)]) -> f64 {
        let mut cost = 0.0;
        for &((kc1, kc2), prop) in bigrams.iter() {
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
                if kc1 == kc2 { SAME_KEY } else { self.bigram_cost[pfing][cfing][jump_len] }
            } else {
                SWITCH_HAND
            };
            cost += percost * prop;
        }
        cost
    }

    #[must_use]
    pub fn trigram_cost(&self, l: &[Kc], trigrams: &[((Kc, Kc, Kc), f64)]) -> f64 {
        const ALT_ROLL_BONUS: f64 = -1.0;
        let mut cost = 0.0;
        for &((kc1, kc2, kc3), prop) in trigrams.iter() {
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

    #[must_use]
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

    #[must_use]
    pub fn without_fixed(&self, inp: &[Kc]) -> Vec<Kc> {
        assert_eq!(inp.len(), self.universe.len(), "must have same size when removing fixed");
        let mut out: Vec<Kc> = Vec::with_capacity(self.universe.len());
        for (i, v) in inp.iter().enumerate() {
            if self.fixed[i] == Kc::None {
                out.push(*v);
            }
        }
        out
    }

    #[must_use]
    pub fn with_fixed(&self, inp: &[Kc]) -> Vec<Kc> {
        let mut out: Vec<Kc> = Vec::with_capacity(self.universe.len());
        let mut idx = 0;
        for i in 0..self.universe.len() {
            if self.fixed[i] == Kc::None {
                out.push(inp[idx]);
                idx += 1;
            } else {
                out.push(self.fixed[i]);
            }
        }
        assert_eq!(idx, inp.len(), "left over keys");
        out
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_key_relation() {
        let model = Model {
            row: vec![2, 2, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0],
            hand: vec![0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1],
            finger: vec![1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1],
            ..Default::default()
        };
        assert_eq!(model.key_below(0), Some(4));
        assert_eq!(model.key_below(1), Some(5));
        assert_eq!(model.key_below(2), Some(6));
        assert_eq!(model.key_below(3), Some(7));
        assert_eq!(model.key_below(4), Some(8));
        assert_eq!(model.key_below(5), Some(9));
        assert_eq!(model.key_below(6), Some(10));
        assert_eq!(model.key_below(7), Some(11));
        assert_eq!(model.key_below(8), None);
        assert_eq!(model.key_below(9), None);
        assert_eq!(model.key_below(10), None);
        assert_eq!(model.key_below(11), None);
    }

    #[test]
    fn test_unigrams() {
        let model = Model { unigram_cost: vec![1.0, 10.0], ..Default::default() };
        let l = &[Kc::A, Kc::B];
        assert_relative_eq!(13.0, model.unigram_cost(l, &[(Kc::A, 3.0), (Kc::B, 1.0)]));
        assert_relative_eq!(PENALTY * 3.0, model.unigram_cost(l, &[(Kc::C, 3.0)]));
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
                    [0.1, 1.7, 0.0, 3.5, 4.5], // Middle - same row val only used for different key locations
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
            unigram_cost: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0],
            row: vec![2, 2, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0],
            hand: vec![0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1],
            finger: vec![1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1],
            ..Default::default()
        };
        // 1 0   0 1
        // A B | C D  2
        // E F | G H  1
        // I J | K L  0
        let l =
            &[Kc::A, Kc::B, Kc::C, Kc::D, Kc::E, Kc::F, Kc::G, Kc::H, Kc::I, Kc::J, Kc::K, Kc::L];
        // Unigram cost test
        assert_relative_eq!(1.0, model.unigram_cost(l, &[(Kc::A, 1.0)]));
        assert_relative_eq!(2.0, model.unigram_cost(l, &[(Kc::B, 1.0)]));
        assert_relative_eq!(3.0, model.unigram_cost(l, &[(Kc::C, 1.0)]));
        assert_relative_eq!(4.0, model.unigram_cost(l, &[(Kc::D, 1.0)]));
        assert_relative_eq!(1.0, model.unigram_cost(l, &[(Kc::B, 0.5)]));
        assert_relative_eq!(3.0, model.unigram_cost(l, &[(Kc::B, 0.5), (Kc::D, 0.5)]));

        // Bigram cost test
        // Middle down 2 middle
        assert_relative_eq!(0.1, model.bigram_cost(l, &[((Kc::A, Kc::I), 1.0)]));
        assert_relative_eq!(0.1, model.bigram_cost(l, &[((Kc::D, Kc::L), 1.0)]));
        // Middle down 1 middle
        assert_relative_eq!(1.7, model.bigram_cost(l, &[((Kc::A, Kc::E), 1.0)]));
        assert_relative_eq!(1.7, model.bigram_cost(l, &[((Kc::D, Kc::H), 1.0)]));
        // Middle up 1 middle
        assert_relative_eq!(3.5, model.bigram_cost(l, &[((Kc::E, Kc::A), 1.0)]));
        assert_relative_eq!(3.5, model.bigram_cost(l, &[((Kc::H, Kc::D), 1.0)]));
        // Middle up 2 middle
        assert_relative_eq!(4.5, model.bigram_cost(l, &[((Kc::I, Kc::A), 1.0)]));
        assert_relative_eq!(4.5, model.bigram_cost(l, &[((Kc::L, Kc::D), 1.0)]));
        // Middle up 1 index
        assert_relative_eq!(-0.5, model.bigram_cost(l, &[((Kc::H, Kc::C), 1.0)]));
        // Middle up 2 index
        assert_relative_eq!(1.5, model.bigram_cost(l, &[((Kc::L, Kc::C), 1.0)]));

        assert_relative_eq!(
            0.875,
            model.bigram_cost(l, &[((Kc::L, Kc::D), 0.25), ((Kc::H, Kc::C), 0.5)])
        );

        // Trigram cost test
        assert_relative_eq!(-1.0, model.trigram_cost(l, &[((Kc::A, Kc::B, Kc::C), 1.0)]));
        assert_relative_eq!(0.0, model.trigram_cost(l, &[((Kc::B, Kc::A, Kc::C), 1.0)]));
    }
}
