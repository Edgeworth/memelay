use derive_more::{Deref, DerefMut, Display};
use eyre::Result;
use memega::eval::Evaluator;
use memega::ops::crossover::{crossover_cycle, crossover_order, crossover_pmx};
use memega::ops::distance::count_different;
use memega::ops::mutation::{mutate_insert, mutate_inversion, mutate_scramble, mutate_swap};
use rand::Rng;

use crate::Args;
use crate::ingest::{load_histograms, load_model};
use crate::model::{Model, PENALTY};
use crate::types::{COLEMAK_DHM, Kc};

#[must_use]
#[derive(Debug, Clone)]
pub struct Histograms {
    pub unigrams: Vec<(Kc, f64)>,
    pub bigrams: Vec<((Kc, Kc), f64)>,
    pub trigrams: Vec<((Kc, Kc, Kc), f64)>,
}

#[must_use]
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

#[must_use]
#[derive(Debug, Display, Deref, DerefMut, Hash, Clone, PartialEq, Eq, PartialOrd)]
#[display("{_0:?}")]
pub struct KeyState(pub Vec<Kc>);

impl Evaluator for LayoutEval {
    type State = KeyState;
    const NUM_CROSSOVER: usize = 4;
    const NUM_MUTATION: usize = 4;

    fn crossover(&self, s1: &mut Self::State, s2: &mut Self::State, idx: usize) {
        // Crossover without touching fixed keys.
        let mut unfixed1 = self.model.without_fixed(s1);
        let mut unfixed2 = self.model.without_fixed(s2);
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
        }
        *s1 = KeyState(self.model.with_fixed(&unfixed1));
        *s2 = KeyState(self.model.with_fixed(&unfixed2));
    }

    fn mutate(&self, s: &mut Self::State, rate: f64, idx: usize) {
        let mut r = rand::rng();
        let mutate = r.random::<f64>() < rate;
        // Mutate without touching fixed keys.
        let mut unfixed = self.model.without_fixed(s);
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
        *s = KeyState(self.model.with_fixed(&unfixed));
    }

    fn fitness(&self, s: &Self::State, _data: &Self::Data) -> Result<f64> {
        let mut cost = 0.0;

        cost += self.model.unigram_cost(s, &self.hist.unigrams);
        cost += self.model.bigram_cost(s, &self.hist.bigrams);
        cost += self.model.trigram_cost(s, &self.hist.trigrams);

        #[must_use]
        struct Cons {
            a: Kc,
            b: Kc,
            horiz: bool,
            ordered: bool,
        }

        let horiz = [
            Cons { a: Kc::Comma, b: Kc::Dot, horiz: true, ordered: true }, // Keep , and . next to eachother.
            Cons { a: Kc::DoubleQuote, b: Kc::Quote, horiz: false, ordered: false }, // " and ' vert.
            Cons { a: Kc::Asterisk, b: Kc::Ampersand, horiz: false, ordered: false }, // * and & vert.
            Cons { a: Kc::Ampersand, b: Kc::Bar, horiz: false, ordered: false }, // | and & vert.
            Cons { a: Kc::Minus, b: Kc::Plus, horiz: false, ordered: false },    // - and + vert.
        ];
        for Cons { a, b, horiz, ordered } in horiz {
            let apos = s.iter().position(|&v| v == a);
            let bpos = s.iter().position(|&v| v == b);
            if bpos.is_none() || apos.is_none() {
                continue;
            }
            let apos = apos.unwrap();
            let bpos = bpos.unwrap();
            let (ab, ba) = if horiz {
                (apos + 1 == bpos, bpos + 1 == apos)
            } else {
                let abelow = self.model.key_below(apos);
                let bbelow = self.model.key_below(bpos);
                let ab = if let Some(abelow) = abelow { abelow == bpos } else { false };
                let ba = if let Some(bbelow) = bbelow { bbelow == apos } else { false };
                (ab, ba)
            };
            if (!ba || ordered) && !ab {
                cost += PENALTY;
            }
        }

        // Check fixed keys
        for (i, &kc) in self.model.fixed.iter().enumerate() {
            if kc != Kc::None && kc != s[i] {
                cost += PENALTY;
            }
        }

        // Tie-breaking: similarity to given existing layout:
        cost += count_different(s, &self.match_keys) as f64 / 100000.0;

        // 1.0 / (cost + 1.0)
        Ok((-cost).exp())
    }

    fn distance(&self, s1: &Self::State, s2: &Self::State) -> Result<f64> {
        let mut d = 0.0;
        for i in 0..s1.len() {
            d += (i8::from(s1[i]) - i8::from(s2[i])).abs() as f64;
        }
        Ok(d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use crate::model::Model;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_key_state_new() {
        let state = KeyState(vec![Kc::A, Kc::B, Kc::C]);
        assert_eq!(state.0, vec![Kc::A, Kc::B, Kc::C]);
    }

    #[test]
    fn test_key_state_deref() {
        let state = KeyState(vec![Kc::A, Kc::B, Kc::C]);
        assert_eq!(state.len(), 3);
        assert_eq!(state[0], Kc::A);
    }

    #[test]
    fn test_key_state_deref_mut() {
        let mut state = KeyState(vec![Kc::A, Kc::B, Kc::C]);
        state[0] = Kc::Z;
        assert_eq!(state[0], Kc::Z);
    }

    #[test]
    fn test_key_state_clone() {
        let state1 = KeyState(vec![Kc::A, Kc::B]);
        let state2 = state1.clone();
        assert_eq!(state1, state2);
    }

    #[test]
    fn test_key_state_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        let state1 = KeyState(vec![Kc::A, Kc::B]);
        let state2 = KeyState(vec![Kc::A, Kc::B]);
        let state3 = KeyState(vec![Kc::B, Kc::A]);
        set.insert(state1.clone());
        set.insert(state2);
        set.insert(state3);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_key_state_display() {
        let state = KeyState(vec![Kc::A, Kc::B, Kc::C]);
        let display_str = format!("{}", state);
        assert!(display_str.contains("A"));
        assert!(display_str.contains("B"));
        assert!(display_str.contains("C"));
    }

    #[test]
    fn test_histograms_clone() {
        let hist = Histograms {
            unigrams: vec![(Kc::A, 1.0)],
            bigrams: vec![((Kc::A, Kc::B), 2.0)],
            trigrams: vec![((Kc::A, Kc::B, Kc::C), 3.0)],
        };
        let cloned = hist.clone();
        assert_eq!(hist.unigrams.len(), cloned.unigrams.len());
        assert_eq!(hist.bigrams.len(), cloned.bigrams.len());
        assert_eq!(hist.trigrams.len(), cloned.trigrams.len());
    }

    #[test]
    fn test_evaluator_crossover_noop() {
        let model = Model {
            universe: vec![Kc::A, Kc::B, Kc::C],
            fixed: vec![Kc::None, Kc::None, Kc::None],
            ..Default::default()
        };
        let eval = LayoutEval {
            model,
            match_keys: vec![],
            hist: Histograms {
                unigrams: vec![],
                bigrams: vec![],
                trigrams: vec![],
            },
        };

        let mut s1 = KeyState(vec![Kc::A, Kc::B, Kc::C]);
        let mut s2 = KeyState(vec![Kc::C, Kc::B, Kc::A]);
        let s1_orig = s1.clone();
        let s2_orig = s2.clone();

        eval.crossover(&mut s1, &mut s2, 0);

        assert_eq!(s1, s1_orig);
        assert_eq!(s2, s2_orig);
    }

    #[test]
    fn test_evaluator_mutate_with_zero_rate() {
        let model = Model {
            universe: vec![Kc::A, Kc::B, Kc::C],
            fixed: vec![Kc::None, Kc::None, Kc::None],
            ..Default::default()
        };
        let eval = LayoutEval {
            model,
            match_keys: vec![],
            hist: Histograms {
                unigrams: vec![],
                bigrams: vec![],
                trigrams: vec![],
            },
        };

        let mut s = KeyState(vec![Kc::A, Kc::B, Kc::C]);
        let original = s.clone();

        for idx in 0..4 {
            eval.mutate(&mut s, 0.0, idx);
        }

        assert_eq!(s, original);
    }

    #[test]
    fn test_evaluator_distance_same_state() {
        let model = Model {
            universe: vec![Kc::A, Kc::B, Kc::C],
            ..Default::default()
        };
        let eval = LayoutEval {
            model,
            match_keys: vec![],
            hist: Histograms {
                unigrams: vec![],
                bigrams: vec![],
                trigrams: vec![],
            },
        };

        let s = KeyState(vec![Kc::A, Kc::B, Kc::C]);
        let dist = eval.distance(&s, &s).unwrap();
        assert_relative_eq!(dist, 0.0);
    }

    #[test]
    fn test_evaluator_distance_different_states() {
        let model = Model {
            universe: vec![Kc::A, Kc::B, Kc::C],
            ..Default::default()
        };
        let eval = LayoutEval {
            model,
            match_keys: vec![],
            hist: Histograms {
                unigrams: vec![],
                bigrams: vec![],
                trigrams: vec![],
            },
        };

        let s1 = KeyState(vec![Kc::Num0, Kc::Num1, Kc::Num2]);
        let s2 = KeyState(vec![Kc::Num5, Kc::Num6, Kc::Num7]);
        let dist = eval.distance(&s1, &s2).unwrap();
        assert!(dist > 0.0);
    }

    #[test]
    fn test_evaluator_fitness_empty_histograms() {
        let model = Model {
            universe: vec![Kc::A, Kc::B, Kc::C],
            fixed: vec![Kc::None, Kc::None, Kc::None],
            unigram_cost: vec![1.0, 1.0, 1.0],
            row: vec![0, 0, 0],
            hand: vec![0, 0, 1],
            finger: vec![0, 1, 0],
            bigram_cost: [[[0.0; 5]; 4]; 4],
            ..Default::default()
        };
        let eval = LayoutEval {
            model,
            match_keys: vec![],
            hist: Histograms {
                unigrams: vec![],
                bigrams: vec![],
                trigrams: vec![],
            },
        };

        let s = KeyState(vec![Kc::A, Kc::B, Kc::C]);
        let fitness = eval.fitness(&s, &()).unwrap();
        assert!(fitness > 0.0 && fitness <= 1.0);
    }

    #[test]
    fn test_evaluator_fitness_with_unigrams() {
        let model = Model {
            universe: vec![Kc::A, Kc::B],
            fixed: vec![Kc::None, Kc::None],
            unigram_cost: vec![1.0, 2.0],
            row: vec![0, 0],
            hand: vec![0, 1],
            finger: vec![0, 0],
            bigram_cost: [[[0.0; 5]; 4]; 4],
            ..Default::default()
        };
        let eval = LayoutEval {
            model,
            match_keys: vec![],
            hist: Histograms {
                unigrams: vec![(Kc::A, 1.0), (Kc::B, 1.0)],
                bigrams: vec![],
                trigrams: vec![],
            },
        };

        let s = KeyState(vec![Kc::A, Kc::B]);
        let fitness = eval.fitness(&s, &()).unwrap();
        assert!(fitness > 0.0 && fitness <= 1.0);
    }

    #[test]
    fn test_evaluator_fitness_fixed_key_violation() {
        let model = Model {
            universe: vec![Kc::A, Kc::B],
            fixed: vec![Kc::A, Kc::None],
            unigram_cost: vec![1.0, 1.0],
            row: vec![0, 0],
            hand: vec![0, 1],
            finger: vec![0, 0],
            bigram_cost: [[[0.0; 5]; 4]; 4],
            ..Default::default()
        };
        let eval = LayoutEval {
            model,
            match_keys: vec![],
            hist: Histograms {
                unigrams: vec![],
                bigrams: vec![],
                trigrams: vec![],
            },
        };

        let s_good = KeyState(vec![Kc::A, Kc::B]);
        let s_bad = KeyState(vec![Kc::B, Kc::A]);

        let fitness_good = eval.fitness(&s_good, &()).unwrap();
        let fitness_bad = eval.fitness(&s_bad, &()).unwrap();

        assert!(fitness_good > fitness_bad);
    }

    #[test]
    fn test_evaluator_fitness_comma_dot_horizontal() {
        let model = Model {
            universe: vec![Kc::Comma, Kc::Dot, Kc::A, Kc::B],
            fixed: vec![Kc::None, Kc::None, Kc::None, Kc::None],
            unigram_cost: vec![1.0, 1.0, 1.0, 1.0],
            row: vec![0, 0, 0, 0],
            hand: vec![0, 0, 0, 0],
            finger: vec![0, 1, 2, 3],
            bigram_cost: [[[0.0; 5]; 4]; 4],
            ..Default::default()
        };
        let eval = LayoutEval {
            model,
            match_keys: vec![],
            hist: Histograms {
                unigrams: vec![],
                bigrams: vec![],
                trigrams: vec![],
            },
        };

        let s_good = KeyState(vec![Kc::Comma, Kc::Dot, Kc::A, Kc::B]);
        let s_bad = KeyState(vec![Kc::Dot, Kc::A, Kc::Comma, Kc::B]);

        let fitness_good = eval.fitness(&s_good, &()).unwrap();
        let fitness_bad = eval.fitness(&s_bad, &()).unwrap();

        assert!(fitness_good > fitness_bad);
    }

    #[test]
    fn test_evaluator_fitness_vertical_constraints() {
        let model = Model {
            universe: vec![Kc::DoubleQuote, Kc::Quote, Kc::A, Kc::B],
            fixed: vec![Kc::None, Kc::None, Kc::None, Kc::None],
            unigram_cost: vec![1.0, 1.0, 1.0, 1.0],
            row: vec![1, 0, 1, 0],
            hand: vec![0, 0, 1, 1],
            finger: vec![0, 0, 0, 0],
            bigram_cost: [[[0.0; 5]; 4]; 4],
            ..Default::default()
        };
        let eval = LayoutEval {
            model,
            match_keys: vec![],
            hist: Histograms {
                unigrams: vec![],
                bigrams: vec![],
                trigrams: vec![],
            },
        };

        let s_good = KeyState(vec![Kc::DoubleQuote, Kc::Quote, Kc::A, Kc::B]);
        let s_bad = KeyState(vec![Kc::Quote, Kc::DoubleQuote, Kc::A, Kc::B]);

        let fitness_good = eval.fitness(&s_good, &()).unwrap();
        let fitness_bad = eval.fitness(&s_bad, &()).unwrap();

        assert!(fitness_good >= fitness_bad);
    }

    #[test]
    fn test_evaluator_num_crossover() {
        assert_eq!(LayoutEval::NUM_CROSSOVER, 4);
    }

    #[test]
    fn test_evaluator_num_mutation() {
        assert_eq!(LayoutEval::NUM_MUTATION, 4);
    }

    #[test]
    fn test_key_state_ordering() {
        let s1 = KeyState(vec![Kc::A, Kc::B]);
        let s2 = KeyState(vec![Kc::A, Kc::C]);
        let s3 = KeyState(vec![Kc::B, Kc::A]);

        assert!(s1 < s2);
        assert!(s1 < s3);
        assert!(s2 < s3);
    }

    #[test]
    fn test_evaluator_crossover_all_strategies() {
        let model = Model {
            universe: vec![Kc::A, Kc::B, Kc::C, Kc::D],
            fixed: vec![Kc::None, Kc::None, Kc::None, Kc::None],
            ..Default::default()
        };
        let eval = LayoutEval {
            model,
            match_keys: vec![],
            hist: Histograms {
                unigrams: vec![],
                bigrams: vec![],
                trigrams: vec![],
            },
        };

        for idx in 0..4 {
            let mut s1 = KeyState(vec![Kc::A, Kc::B, Kc::C, Kc::D]);
            let mut s2 = KeyState(vec![Kc::D, Kc::C, Kc::B, Kc::A]);
            eval.crossover(&mut s1, &mut s2, idx);
            assert_eq!(s1.len(), 4);
            assert_eq!(s2.len(), 4);
        }
    }

    #[test]
    fn test_evaluator_mutate_all_strategies() {
        let model = Model {
            universe: vec![Kc::A, Kc::B, Kc::C, Kc::D],
            fixed: vec![Kc::None, Kc::None, Kc::None, Kc::None],
            ..Default::default()
        };
        let eval = LayoutEval {
            model,
            match_keys: vec![],
            hist: Histograms {
                unigrams: vec![],
                bigrams: vec![],
                trigrams: vec![],
            },
        };

        for idx in 0..4 {
            let mut s = KeyState(vec![Kc::A, Kc::B, Kc::C, Kc::D]);
            eval.mutate(&mut s, 1.0, idx);
            assert_eq!(s.len(), 4);
        }
    }

    #[test]
    fn test_evaluator_crossover_with_fixed_keys() {
        let model = Model {
            universe: vec![Kc::A, Kc::B, Kc::C],
            fixed: vec![Kc::A, Kc::None, Kc::None],
            ..Default::default()
        };
        let eval = LayoutEval {
            model,
            match_keys: vec![],
            hist: Histograms {
                unigrams: vec![],
                bigrams: vec![],
                trigrams: vec![],
            },
        };

        let mut s1 = KeyState(vec![Kc::A, Kc::B, Kc::C]);
        let mut s2 = KeyState(vec![Kc::A, Kc::C, Kc::B]);

        eval.crossover(&mut s1, &mut s2, 1);

        assert_eq!(s1[0], Kc::A);
        assert_eq!(s2[0], Kc::A);
    }

    #[test]
    fn test_evaluator_mutate_with_fixed_keys() {
        let model = Model {
            universe: vec![Kc::A, Kc::B, Kc::C],
            fixed: vec![Kc::A, Kc::None, Kc::None],
            ..Default::default()
        };
        let eval = LayoutEval {
            model,
            match_keys: vec![],
            hist: Histograms {
                unigrams: vec![],
                bigrams: vec![],
                trigrams: vec![],
            },
        };

        let mut s = KeyState(vec![Kc::A, Kc::B, Kc::C]);
        eval.mutate(&mut s, 1.0, 0);

        assert_eq!(s[0], Kc::A);
    }

    #[test]
    fn test_evaluator_fitness_with_bigrams() {
        let model = Model {
            universe: vec![Kc::A, Kc::B, Kc::C],
            fixed: vec![Kc::None, Kc::None, Kc::None],
            unigram_cost: vec![1.0, 1.0, 1.0],
            row: vec![0, 0, 1],
            hand: vec![0, 1, 0],
            finger: vec![0, 0, 1],
            bigram_cost: [[[1.0; 5]; 4]; 4],
            ..Default::default()
        };
        let eval = LayoutEval {
            model,
            match_keys: vec![],
            hist: Histograms {
                unigrams: vec![],
                bigrams: vec![((Kc::A, Kc::B), 1.0)],
                trigrams: vec![],
            },
        };

        let s = KeyState(vec![Kc::A, Kc::B, Kc::C]);
        let fitness = eval.fitness(&s, &()).unwrap();
        assert!(fitness > 0.0);
    }

    #[test]
    fn test_evaluator_fitness_with_trigrams() {
        let model = Model {
            universe: vec![Kc::A, Kc::B, Kc::C],
            fixed: vec![Kc::None, Kc::None, Kc::None],
            unigram_cost: vec![1.0, 1.0, 1.0],
            row: vec![0, 0, 0],
            hand: vec![0, 0, 1],
            finger: vec![0, 1, 0],
            bigram_cost: [[[0.0; 5]; 4]; 4],
            ..Default::default()
        };
        let eval = LayoutEval {
            model,
            match_keys: vec![],
            hist: Histograms {
                unigrams: vec![],
                bigrams: vec![],
                trigrams: vec![((Kc::A, Kc::B, Kc::C), 1.0)],
            },
        };

        let s = KeyState(vec![Kc::A, Kc::B, Kc::C]);
        let fitness = eval.fitness(&s, &()).unwrap();
        assert!(fitness > 0.0 && fitness <= 1.0);
    }

    #[test]
    fn test_histograms_debug() {
        let hist = Histograms {
            unigrams: vec![],
            bigrams: vec![],
            trigrams: vec![],
        };
        let debug_str = format!("{:?}", hist);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_key_state_equality() {
        let s1 = KeyState(vec![Kc::A, Kc::B]);
        let s2 = KeyState(vec![Kc::A, Kc::B]);
        let s3 = KeyState(vec![Kc::B, Kc::A]);
        assert_eq!(s1, s2);
        assert_ne!(s1, s3);
    }

    #[test]
    fn test_evaluator_distance_symmetric() {
        let model = Model {
            universe: vec![Kc::A, Kc::B, Kc::C],
            ..Default::default()
        };
        let eval = LayoutEval {
            model,
            match_keys: vec![],
            hist: Histograms {
                unigrams: vec![],
                bigrams: vec![],
                trigrams: vec![],
            },
        };

        let s1 = KeyState(vec![Kc::Num0, Kc::Num1, Kc::Num2]);
        let s2 = KeyState(vec![Kc::Num5, Kc::Num6, Kc::Num7]);

        let dist1 = eval.distance(&s1, &s2).unwrap();
        let dist2 = eval.distance(&s2, &s1).unwrap();

        assert_relative_eq!(dist1, dist2);
    }
}
